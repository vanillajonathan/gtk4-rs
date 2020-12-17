// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::Gesture;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct GestureRotate(Object<ffi::GtkGestureRotate, ffi::GtkGestureRotateClass>) @extends Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_rotate_get_type(),
    }
}

impl GestureRotate {
    #[doc(alias = "gtk_gesture_rotate_new")]
    pub fn new() -> GestureRotate {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(ffi::gtk_gesture_rotate_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_gesture_rotate_get_angle_delta")]
    pub fn get_angle_delta(&self) -> f64 {
        unsafe { ffi::gtk_gesture_rotate_get_angle_delta(self.to_glib_none().0) }
    }

    pub fn connect_angle_changed<F: Fn(&GestureRotate, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn angle_changed_trampoline<F: Fn(&GestureRotate, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureRotate,
            angle: libc::c_double,
            angle_delta: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), angle, angle_delta)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"angle-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    angle_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureRotate {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GestureRotate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GestureRotate")
    }
}
