// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Window;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "GtkUriLauncher")]
    pub struct UriLauncher(Object<ffi::GtkUriLauncher, ffi::GtkUriLauncherClass>);

    match fn {
        type_ => || ffi::gtk_uri_launcher_get_type(),
    }
}

impl UriLauncher {
    #[doc(alias = "gtk_uri_launcher_new")]
    pub fn new(uri: &str) -> UriLauncher {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_uri_launcher_new(uri.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`UriLauncher`] objects.
    ///
    /// This method returns an instance of [`UriLauncherBuilder`](crate::builders::UriLauncherBuilder) which can be used to create [`UriLauncher`] objects.
    pub fn builder() -> UriLauncherBuilder {
        UriLauncherBuilder::new()
    }

    #[doc(alias = "gtk_uri_launcher_get_uri")]
    #[doc(alias = "get_uri")]
    pub fn uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_uri_launcher_get_uri(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_uri_launcher_launch")]
    pub fn launch<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        parent: Option<&impl IsA<Window>>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn launch_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_uri_launcher_launch_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = launch_trampoline::<P>;
        unsafe {
            ffi::gtk_uri_launcher_launch(
                self.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn launch_future(
        &self,
        parent: Option<&(impl IsA<Window> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let parent = parent.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.launch(
                parent.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "gtk_uri_launcher_set_uri")]
    pub fn set_uri(&self, uri: Option<&str>) {
        unsafe {
            ffi::gtk_uri_launcher_set_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "uri")]
    pub fn connect_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uri_trampoline<F: Fn(&UriLauncher) + 'static>(
            this: *mut ffi::GtkUriLauncher,
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
                b"notify::uri\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_uri_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
impl Default for UriLauncher {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`UriLauncher`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct UriLauncherBuilder {
    builder: glib::object::ObjectBuilder<'static, UriLauncher>,
}

impl UriLauncherBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn uri(self, uri: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("uri", uri.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`UriLauncher`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> UriLauncher {
        self.builder.build()
    }
}

impl fmt::Display for UriLauncher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UriLauncher")
    }
}
