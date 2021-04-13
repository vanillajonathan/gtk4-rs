// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

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
    pub struct TreeListRow(Object<ffi::GtkTreeListRow, ffi::GtkTreeListRowClass>);

    match fn {
        get_type => || ffi::gtk_tree_list_row_get_type(),
    }
}

impl TreeListRow {
    #[doc(alias = "gtk_tree_list_row_get_child_row")]
    pub fn get_child_row(&self, position: u32) -> Option<TreeListRow> {
        unsafe {
            from_glib_full(ffi::gtk_tree_list_row_get_child_row(
                self.to_glib_none().0,
                position,
            ))
        }
    }

    #[doc(alias = "gtk_tree_list_row_get_children")]
    pub fn children(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_tree_list_row_get_children(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_list_row_get_depth")]
    pub fn depth(&self) -> u32 {
        unsafe { ffi::gtk_tree_list_row_get_depth(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_tree_list_row_get_expanded")]
    pub fn is_expanded(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_list_row_get_expanded(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_list_row_get_item")]
    pub fn item(&self) -> Option<glib::Object> {
        unsafe { from_glib_full(ffi::gtk_tree_list_row_get_item(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_list_row_get_parent")]
    pub fn parent(&self) -> Option<TreeListRow> {
        unsafe { from_glib_full(ffi::gtk_tree_list_row_get_parent(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_list_row_get_position")]
    pub fn position(&self) -> u32 {
        unsafe { ffi::gtk_tree_list_row_get_position(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_tree_list_row_is_expandable")]
    pub fn is_expandable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_list_row_is_expandable(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_list_row_set_expanded")]
    pub fn set_expanded(&self, expanded: bool) {
        unsafe {
            ffi::gtk_tree_list_row_set_expanded(self.to_glib_none().0, expanded.to_glib());
        }
    }

    pub fn connect_property_children_notify<F: Fn(&TreeListRow) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_children_trampoline<F: Fn(&TreeListRow) + 'static>(
            this: *mut ffi::GtkTreeListRow,
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
                b"notify::children\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_children_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_depth_notify<F: Fn(&TreeListRow) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_depth_trampoline<F: Fn(&TreeListRow) + 'static>(
            this: *mut ffi::GtkTreeListRow,
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
                b"notify::depth\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_depth_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_expandable_notify<F: Fn(&TreeListRow) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_expandable_trampoline<F: Fn(&TreeListRow) + 'static>(
            this: *mut ffi::GtkTreeListRow,
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
                b"notify::expandable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expandable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_expanded_notify<F: Fn(&TreeListRow) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_expanded_trampoline<F: Fn(&TreeListRow) + 'static>(
            this: *mut ffi::GtkTreeListRow,
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
                b"notify::expanded\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expanded_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_item_notify<F: Fn(&TreeListRow) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_item_trampoline<F: Fn(&TreeListRow) + 'static>(
            this: *mut ffi::GtkTreeListRow,
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
                b"notify::item\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_item_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct TreeListRowBuilder {
    expanded: Option<bool>,
}

impl TreeListRowBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> TreeListRow {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref expanded) = self.expanded {
            properties.push(("expanded", expanded));
        }
        let ret = glib::Object::new::<TreeListRow>(&properties).expect("object new");
        ret
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some(expanded);
        self
    }
}

impl fmt::Display for TreeListRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeListRow")
    }
}
