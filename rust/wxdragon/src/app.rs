// Application lifecycle wrapper
// Currently, the main application logic is driven by the C wxd_Main function.
// This module might later contain wrappers for App-specific functions if needed.

use crate::window::WxWidget;
use std::ffi::{c_char, c_void, CString};
use wxdragon_sys as ffi; // Import Window and WxWidget trait
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use lazy_static::lazy_static;

// Queue for storing callbacks to be executed on the main thread
lazy_static! {
    static ref MAIN_THREAD_QUEUE: Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send + 'static>>>> = 
        Arc::new(Mutex::new(VecDeque::new()));
}

/// Schedules a callback to be executed on the main thread.
/// 
/// This is useful when you need to update UI elements from a background thread.
/// The callback will be executed during the next event loop iteration.
/// 
/// # Example
/// ```
/// use wxdragon::prelude::*;
/// 
/// // In a background thread:
/// wxdragon::call_after(Box::new(move || {
///     // Update UI elements here
///     my_label.set_label("Updated from background thread");
/// }));
/// ```
pub fn call_after<F>(callback: Box<F>) 
where 
    F: FnOnce() + Send + 'static
{
    let mut queue = MAIN_THREAD_QUEUE.lock().unwrap();
    queue.push_back(callback);
}

/// Processes pending callbacks queued via `call_after`.
/// 
/// This function is called automatically by the event loop.
/// You do not need to call this function manually.
pub fn process_main_thread_queue() {
    let mut callbacks = Vec::new();
    
    // Move callbacks from the queue to our local vector to minimize lock time
    {
        let mut queue = MAIN_THREAD_QUEUE.lock().unwrap();
        if queue.is_empty() {
            return;
        }
        
        // Move up to 10 callbacks at a time to prevent UI freezes
        // if there are many callbacks pending
        for _ in 0..10 {
            if let Some(callback) = queue.pop_front() {
                callbacks.push(callback);
            } else {
                break;
            }
        }
    }
    
    // Execute callbacks outside of the lock
    for callback in callbacks {
        callback();
    }
}

// This function is called from C++ to process pending callbacks
#[no_mangle]
pub extern "C" fn process_rust_callbacks() {
    process_main_thread_queue();
}

// Function to manually trigger callback processing (useful for tests)
pub fn process_callbacks() {
    unsafe {
        ffi::wxd_App_ProcessCallbacks();
    }
}

/// A handle provided to the `on_init` closure in `wxdragon::main`.
/// Use this handle to register top-level widgets that need to survive
/// beyond the scope of the closure.
#[derive(Default)]
pub struct WxdAppHandle {
    /// Internal list of widgets to preserve.
    widgets_to_preserve: Vec<Box<dyn WxWidget>>,
    
    /// Internal list of arbitrary objects to preserve
    objects_to_preserve: Vec<Box<dyn std::any::Any + Send + 'static>>,
}

impl WxdAppHandle {
    /// Registers a widget to be preserved by the application lifecycle.
    ///
    /// Call this method for any top-level widget (like a `Frame`)
    /// that you create in the `on_init` closure passed to `wxdragon::main`.
    /// This prevents the widget's Rust wrapper from being dropped prematurely
    /// when the closure finishes, transferring its lifetime management to the
    /// underlying wxWidgets event loop.
    ///
    /// Takes ownership of the widget.
    pub fn preserve<W>(&mut self, widget: W)
    where
        W: WxWidget + 'static,
    {
        self.widgets_to_preserve.push(Box::new(widget));
    }
    
    /// Preserves any Rust object in the application lifetime.
    /// 
    /// This is useful for objects like a Tokio runtime that need to 
    /// survive beyond the initialization function.
    pub fn preserve_box<T>(&mut self, object: Box<T>)
    where
        T: std::any::Any + Send + 'static,
    {
        self.objects_to_preserve.push(object);
    }

    /// Sets the application's top window.
    ///
    /// This is necessary for the main event loop to run correctly.
    /// Call this after creating your main Frame.
    pub fn set_top_window<W>(&self, window: &W)
    where
        W: WxWidget + ?Sized, // Allow passing references to trait objects
    {
        let app_ptr = unsafe { ffi::wxd_GetApp() };
        if !app_ptr.is_null() {
            unsafe {
                ffi::wxd_App_SetTopWindow(app_ptr, window.handle_ptr());
            }
        }
    }
}

/// Runs the wxWidgets application main loop, providing a safe entry point.
///
/// This function initializes wxWidgets and starts the event loop. It takes a closure
/// `on_init` that will be called once after basic initialization but before the
/// main event loop begins. The closure receives a mutable `WxdAppHandle` which
/// should be used to register any top-level widgets using `handle.preserve()`.
///
/// The `on_init` closure should return `true` to continue starting the application,
/// or `false` to indicate an initialization error, which will terminate the app cleanly.
///
/// # Panics
/// Panics if the program name cannot be converted to a CString (e.g., contains null bytes).
///
/// # Example
/// ```no_run
/// use wxdragon::frame::Frame;
/// use wxdragon::{self, ID_ANY, WxdAppHandle};
/// use wxdragon::window::WxWidget;
///
/// fn main() {
///     wxdragon::main(|handle: &mut WxdAppHandle| {
///         println!("App initializing!");
///         let frame = Frame::new_simple(None, ID_ANY, "My App").expect("Frame creation failed");
///         frame.show(true);
///         // Register frame to prevent premature drop
///         handle.preserve(frame);
///         true // Indicate success
///     });
/// }
/// ```
pub fn main<F>(on_init: F)
where
    F: FnOnce(&mut WxdAppHandle) -> bool + 'static, // Closure takes handle, returns success
{
    // Box the closure.
    let on_init_boxed: Box<Box<dyn FnOnce(&mut WxdAppHandle) -> bool>> =
        Box::new(Box::new(on_init));
    let user_data_ptr = Box::into_raw(on_init_boxed) as *mut c_void;

    // Prepare arguments for wxd_Main (using a default program name)
    let exit_code = unsafe {
        let prog_name = CString::new("wxRustApp").expect("Failed to create CString for app name");
        let mut argv: [*mut c_char; 2] = [prog_name.into_raw(), std::ptr::null_mut()];
        let argc: i32 = 1;

        // Call the C entry point, passing the trampoline and the closure data
        let result = ffi::wxd_Main(
            argc,
            argv.as_mut_ptr(),
            Some(trampoline_init::<F>), // Pass the generic trampoline
            user_data_ptr,              // Pass the boxed closure as user_data
        );

        let _ = CString::from_raw(argv[0]);
        result
    };

    if exit_code != 0 {
        // eprintln!("Application exited with code: {}", exit_code);
    }
}

// Trampoline function to call the Rust closure from C
unsafe extern "C" fn trampoline_init<F>(user_data: *mut c_void) -> bool
where
    F: FnOnce(&mut WxdAppHandle) -> bool + 'static,
{
    // Reconstruct the Box containing the closure
    let closure_box: Box<Box<dyn FnOnce(&mut WxdAppHandle) -> bool>> =
        Box::from_raw(user_data as *mut _);

    // Create the handle that the closure will use
    let mut handle = WxdAppHandle::default();

    // Call the closure, catching potential panics
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        (*closure_box)(&mut handle) // Call the closure itself, passing the handle
    }));

    // Process the result
    match result {
        Ok(success) => {
            if success {
                // Let the handle drop naturally when the application exits if it was successful.
                // The underlying wxWidgets objects' lifetimes are managed by the C++ event loop.
                // The Rust Drop implementations (like Frame::drop) are needed for cleanup
                // *before* the C++ objects are destroyed (e.g., popping event handlers).
                // Forgetting the handle prevented this necessary cleanup.
                true // Indicate success to wxd_Main
            } else {
                // Initialization failed. Allow the handle to be dropped naturally,
                // which will drop the Vec, the Boxes, and the Widgets for cleanup.
                false // Indicate initialization failure
            }
        }
        Err(_) => {
            eprintln!("Panic caught in Rust AppOnInit callback!");
            // Allow the handle to be dropped for cleanup in case of panic.
            false // Indicate failure on panic
        }
    }
}
