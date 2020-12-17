// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Filter;
use glib::object::Cast;
use glib::object::IsA;
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
    pub struct FilterListModel(Object<ffi::GtkFilterListModel, ffi::GtkFilterListModelClass>) @implements gio::ListModel;

    match fn {
        get_type => || ffi::gtk_filter_list_model_get_type(),
    }
}

impl FilterListModel {
    #[doc(alias = "gtk_filter_list_model_new")]
    pub fn new<P: IsA<gio::ListModel>, Q: IsA<Filter>>(
        model: Option<&P>,
        filter: Option<&Q>,
    ) -> FilterListModel {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_filter_list_model_new(
                model.map(|p| p.as_ref()).to_glib_full(),
                filter.map(|p| p.as_ref()).to_glib_full(),
            ))
        }
    }

    #[doc(alias = "gtk_filter_list_model_get_filter")]
    pub fn get_filter(&self) -> Option<Filter> {
        unsafe { from_glib_none(ffi::gtk_filter_list_model_get_filter(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_filter_list_model_get_incremental")]
    pub fn get_incremental(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_filter_list_model_get_incremental(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_filter_list_model_get_model")]
    pub fn get_model(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_filter_list_model_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_filter_list_model_get_pending")]
    pub fn get_pending(&self) -> u32 {
        unsafe { ffi::gtk_filter_list_model_get_pending(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_filter_list_model_set_filter")]
    pub fn set_filter<P: IsA<Filter>>(&self, filter: Option<&P>) {
        unsafe {
            ffi::gtk_filter_list_model_set_filter(
                self.to_glib_none().0,
                filter.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_filter_list_model_set_incremental")]
    pub fn set_incremental(&self, incremental: bool) {
        unsafe {
            ffi::gtk_filter_list_model_set_incremental(
                self.to_glib_none().0,
                incremental.to_glib(),
            );
        }
    }

    #[doc(alias = "gtk_filter_list_model_set_model")]
    pub fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>) {
        unsafe {
            ffi::gtk_filter_list_model_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_filter_notify<F: Fn(&FilterListModel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_trampoline<F: Fn(&FilterListModel) + 'static>(
            this: *mut ffi::GtkFilterListModel,
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
                b"notify::filter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_incremental_notify<F: Fn(&FilterListModel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_incremental_trampoline<F: Fn(&FilterListModel) + 'static>(
            this: *mut ffi::GtkFilterListModel,
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
                b"notify::incremental\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_incremental_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_model_notify<F: Fn(&FilterListModel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&FilterListModel) + 'static>(
            this: *mut ffi::GtkFilterListModel,
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
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_pending_notify<F: Fn(&FilterListModel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pending_trampoline<F: Fn(&FilterListModel) + 'static>(
            this: *mut ffi::GtkFilterListModel,
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
                b"notify::pending\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pending_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct FilterListModelBuilder {
    filter: Option<Filter>,
    incremental: Option<bool>,
    model: Option<gio::ListModel>,
}

impl FilterListModelBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> FilterListModel {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref filter) = self.filter {
            properties.push(("filter", filter));
        }
        if let Some(ref incremental) = self.incremental {
            properties.push(("incremental", incremental));
        }
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        let ret = glib::Object::new::<FilterListModel>(&properties).expect("object new");
        ret
    }

    pub fn filter<P: IsA<Filter>>(mut self, filter: &P) -> Self {
        self.filter = Some(filter.clone().upcast());
        self
    }

    pub fn incremental(mut self, incremental: bool) -> Self {
        self.incremental = Some(incremental);
        self
    }

    pub fn model<P: IsA<gio::ListModel>>(mut self, model: &P) -> Self {
        self.model = Some(model.clone().upcast());
        self
    }
}

impl fmt::Display for FilterListModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FilterListModel")
    }
}
