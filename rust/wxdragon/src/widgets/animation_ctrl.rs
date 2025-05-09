use wxdragon_sys as ffi;
use crate::window::WxWidget;
use crate::base::{Point, Size, ID_ANY};
use crate::event::WxEvtHandler;
use std::ffi::CString;

// Opaque pointer for wxAnimationCtrl
#[derive(Debug, Clone)]
pub struct AnimationCtrl {
    ptr: *mut ffi::wxd_AnimationCtrl_t,
}

impl WxWidget for AnimationCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.ptr as *mut ffi::wxd_Window_t
    }
}

// ADDED: Implement WxEvtHandler for AnimationCtrl
impl WxEvtHandler for AnimationCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.ptr as *mut ffi::wxd_EvtHandler_t // Cast window pointer to event handler pointer
    }
}

impl AnimationCtrl {
    pub fn builder(parent: &impl WxWidget) -> AnimationCtrlBuilder {
        AnimationCtrlBuilder {
            parent: parent.handle_ptr(),
            id: ID_ANY,
            animation_file: "".to_string(),
            pos: Point::new(-1, -1),
            size: Size::new(-1, -1),
            style: 0,
            name: "wxAnimationCtrl".to_string(),
        }
    }

    // Private constructor from raw pointer
    fn from_ptr(ptr: *mut ffi::wxd_AnimationCtrl_t) -> Self {
        AnimationCtrl { ptr }
    }

    pub fn play(&self) -> bool {
        unsafe { ffi::wxd_AnimationCtrl_Play(self.ptr) }
    }

    pub fn stop(&self) {
        unsafe { ffi::wxd_AnimationCtrl_Stop(self.ptr) }
    }

    pub fn is_playing(&self) -> bool {
        unsafe { ffi::wxd_AnimationCtrl_IsPlaying(self.ptr) }
    }

    pub fn load_file(&self, animation_file: &str) -> bool {
        let c_animation_file = CString::new(animation_file).expect("CString::new failed for animation_file");
        unsafe { ffi::wxd_AnimationCtrl_LoadFile(self.ptr, c_animation_file.as_ptr()) }
    }

    pub fn load_from_bytes(&self, data: &[u8]) -> bool {
        if data.is_empty() {
            return false;
        }
        unsafe {
            ffi::wxd_AnimationCtrl_LoadFromBytes(self.ptr, data.as_ptr(), data.len() as usize)
        }
    }
}

// Builder for wxAnimationCtrl
pub struct AnimationCtrlBuilder {
    parent: *mut ffi::wxd_Window_t,
    id: i32,
    animation_file: String,
    pos: Point,
    size: Size,
    style: i64,
    name: String,
}

impl AnimationCtrlBuilder {
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn with_animation_file(mut self, animation_file: &str) -> Self {
        self.animation_file = animation_file.to_string();
        self
    }

    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn build(self) -> AnimationCtrl {
        let c_animation_file = CString::new(self.animation_file.as_str()).expect("CString::new failed for animation_file");
        let c_name = CString::new(self.name.as_str()).expect("CString::new failed for name");

        let ptr = unsafe {
            ffi::wxd_AnimationCtrl_Create(
                self.parent as *mut ffi::wxd_Window_t,
                self.id,
                c_animation_file.as_ptr(),
                self.pos.x,
                self.pos.y,
                self.size.width,
                self.size.height,
                self.style,
                c_name.as_ptr(),
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxAnimationCtrl");
        }
        AnimationCtrl::from_ptr(ptr)
    }
}

impl Drop for AnimationCtrl {
    fn drop(&mut self) {
        // wxWidgets handles destruction of child controls when the parent is destroyed.
        // If this AnimationCtrl was explicitly created and needs to be destroyed before parent,
        // or if it's a top-level window (not applicable here), a Destroy method would be needed.
        // For now, assuming it's a child widget and wxWidgets handles cleanup.
        // unsafe { ffi::wxd_Window_Destroy(self.ptr as *mut ffi::wxd_Window_t); }
    }
} 