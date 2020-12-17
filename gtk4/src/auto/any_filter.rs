// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buildable;
use crate::Filter;
use crate::MultiFilter;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct AnyFilter(Object<ffi::GtkAnyFilter, ffi::GtkAnyFilterClass>) @extends MultiFilter, Filter, @implements gio::ListModel, Buildable;

    match fn {
        get_type => || ffi::gtk_any_filter_get_type(),
    }
}

impl AnyFilter {
    #[doc(alias = "gtk_any_filter_new")]
    pub fn new() -> AnyFilter {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_any_filter_new()) }
    }
}

impl Default for AnyFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AnyFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AnyFilter")
    }
}
