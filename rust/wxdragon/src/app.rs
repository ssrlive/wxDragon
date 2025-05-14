// Application lifecycle wrapper
// Currently, the main application logic is driven by the C wxd_Main function.
// This module might later contain wrappers for App-specific functions if needed.

use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::ffi::{c_char, c_void, CString};
use std::sync::{Arc, Mutex};
use wxdragon_sys as ffi; // Import Window and WxWidget trait

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
    F: FnOnce() + Send + 'static,
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

/// Sets the application's top window.
///
/// This is necessary for the main event loop to run correctly.
/// Call this after creating your main Frame.
pub fn set_top_window<W>(window: &W)
where
    W: crate::window::WxWidget + ?Sized,
{
    let app_ptr = unsafe { ffi::wxd_GetApp() };
    if !app_ptr.is_null() {
        unsafe {
            ffi::wxd_App_SetTopWindow(app_ptr, window.handle_ptr());
        }
    }
}

/// Runs the wxWidgets application main loop, providing a safe entry point.
///
/// This function initializes wxWidgets and starts the event loop. It takes a closure
/// `on_init` that will be called once after basic initialization but before the
/// main event loop begins.
///
/// # Panics
/// Panics if initialization fails or if the program name cannot be converted to a CString.
///
/// # Example
/// ```no_run
/// use wxdragon::prelude::*;
///
/// fn main() {
///     wxdragon::main(|_| {
///         let frame = Frame::builder()
///             .with_title("My App")
///             .build();
///         frame.show(true);
///         
///         // No need to preserve the frame - wxWidgets manages it
///     });
/// }
/// ```
pub fn main<F>(on_init: F)
where
    F: FnOnce(()) -> () + 'static,
{
    // Box the closure
    let on_init_boxed: Box<Box<dyn FnOnce(()) -> ()>> = Box::new(Box::new(on_init));
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
            Some(trampoline_init::<F>),
            user_data_ptr,
        );

        let _ = CString::from_raw(argv[0]);
        result
    };

    if exit_code != 0 {
        panic!("Application exited with code: {}", exit_code);
    }
}

// Trampoline function to call the Rust closure from C
unsafe extern "C" fn trampoline_init<F>(user_data: *mut c_void) -> bool
where
    F: FnOnce(()) -> () + 'static,
{
    // Reconstruct the Box containing the closure
    let closure_box: Box<Box<dyn FnOnce(()) -> ()>> = Box::from_raw(user_data as *mut _);

    // Call the closure, catching potential panics
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        (*closure_box)(()) // Call the closure itself
    }));

    // Process the result
    match result {
        Ok(_) => true, // Always return success if no panic occurred
        Err(_) => {
            eprintln!("Panic caught in Rust AppOnInit callback!");
            false // Indicate failure on panic
        }
    }
}
