// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DrawContext;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib::wrapper! {
    pub struct GLContext(Object<ffi::GdkGLContext>) @extends DrawContext;

    match fn {
        get_type => || ffi::gdk_gl_context_get_type(),
    }
}

impl GLContext {
    #[doc(alias = "gdk_gl_context_get_debug_enabled")]
    pub fn get_debug_enabled(&self) -> bool {
        unsafe { from_glib(ffi::gdk_gl_context_get_debug_enabled(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_gl_context_get_forward_compatible")]
    pub fn get_forward_compatible(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_forward_compatible(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_context_get_required_version")]
    pub fn get_required_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            ffi::gdk_gl_context_get_required_version(
                self.to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            );
            let major = major.assume_init();
            let minor = minor.assume_init();
            (major, minor)
        }
    }

    #[doc(alias = "gdk_gl_context_get_shared_context")]
    pub fn get_shared_context(&self) -> Option<GLContext> {
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_shared_context(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_context_get_use_es")]
    pub fn get_use_es(&self) -> bool {
        unsafe { from_glib(ffi::gdk_gl_context_get_use_es(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_gl_context_get_version")]
    pub fn get_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            ffi::gdk_gl_context_get_version(
                self.to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            );
            let major = major.assume_init();
            let minor = minor.assume_init();
            (major, minor)
        }
    }

    #[doc(alias = "gdk_gl_context_is_legacy")]
    pub fn is_legacy(&self) -> bool {
        unsafe { from_glib(ffi::gdk_gl_context_is_legacy(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_gl_context_make_current")]
    pub fn make_current(&self) {
        unsafe {
            ffi::gdk_gl_context_make_current(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_gl_context_realize")]
    pub fn realize(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gdk_gl_context_realize(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_gl_context_set_debug_enabled")]
    pub fn set_debug_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gdk_gl_context_set_debug_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    #[doc(alias = "gdk_gl_context_set_forward_compatible")]
    pub fn set_forward_compatible(&self, compatible: bool) {
        unsafe {
            ffi::gdk_gl_context_set_forward_compatible(self.to_glib_none().0, compatible.to_glib());
        }
    }

    #[doc(alias = "gdk_gl_context_set_required_version")]
    pub fn set_required_version(&self, major: i32, minor: i32) {
        unsafe {
            ffi::gdk_gl_context_set_required_version(self.to_glib_none().0, major, minor);
        }
    }

    #[doc(alias = "gdk_gl_context_set_use_es")]
    pub fn set_use_es(&self, use_es: i32) {
        unsafe {
            ffi::gdk_gl_context_set_use_es(self.to_glib_none().0, use_es);
        }
    }

    #[doc(alias = "gdk_gl_context_clear_current")]
    pub fn clear_current() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_gl_context_clear_current();
        }
    }

    #[doc(alias = "gdk_gl_context_get_current")]
    pub fn get_current() -> Option<GLContext> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gdk_gl_context_get_current()) }
    }
}

impl fmt::Display for GLContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GLContext")
    }
}
