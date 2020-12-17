// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Display;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct DisplayManager(Object<ffi::GdkDisplayManager>);

    match fn {
        get_type => || ffi::gdk_display_manager_get_type(),
    }
}

impl DisplayManager {
    #[doc(alias = "gdk_display_manager_get_default_display")]
    pub fn get_default_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_display_manager_get_default_display(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_manager_list_displays")]
    pub fn list_displays(&self) -> Vec<Display> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_display_manager_list_displays(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_manager_open_display")]
    pub fn open_display(&self, name: &str) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_display_manager_open_display(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_manager_set_default_display")]
    pub fn set_default_display(&self, display: &Display) {
        unsafe {
            ffi::gdk_display_manager_set_default_display(
                self.to_glib_none().0,
                display.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_display_manager_get")]
    pub fn get() -> Option<DisplayManager> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gdk_display_manager_get()) }
    }

    pub fn connect_display_opened<F: Fn(&DisplayManager, &Display) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn display_opened_trampoline<
            F: Fn(&DisplayManager, &Display) + 'static,
        >(
            this: *mut ffi::GdkDisplayManager,
            display: *mut ffi::GdkDisplay,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(display))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"display-opened\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    display_opened_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_default_display_notify<F: Fn(&DisplayManager) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_display_trampoline<F: Fn(&DisplayManager) + 'static>(
            this: *mut ffi::GdkDisplayManager,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::default-display\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_default_display_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct DisplayManagerBuilder {
    default_display: Option<Display>,
}

impl DisplayManagerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> DisplayManager {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref default_display) = self.default_display {
            properties.push(("default-display", default_display));
        }
        let ret = glib::Object::new::<DisplayManager>(&properties).expect("object new");
        ret
    }

    pub fn default_display(mut self, default_display: &Display) -> Self {
        self.default_display = Some(default_display.clone());
        self
    }
}

impl fmt::Display for DisplayManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DisplayManager")
    }
}
