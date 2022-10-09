// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Accessible;
use crate::AppChooser;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::Dialog;
use crate::DialogFlags;
use crate::Native;
use crate::Root;
use crate::ShortcutManager;
use crate::Widget;
use crate::Window;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkAppChooserDialog")]
    pub struct AppChooserDialog(Object<ffi::GtkAppChooserDialog>) @extends Dialog, Window, Widget, @implements Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager, AppChooser;

    match fn {
        type_ => || ffi::gtk_app_chooser_dialog_get_type(),
    }
}

impl AppChooserDialog {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_app_chooser_dialog_new")]
    pub fn new(
        parent: Option<&impl IsA<Window>>,
        flags: DialogFlags,
        file: &impl IsA<gio::File>,
    ) -> AppChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_dialog_new(
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
                file.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_app_chooser_dialog_new_for_content_type")]
    #[doc(alias = "new_for_content_type")]
    pub fn for_content_type(
        parent: Option<&impl IsA<Window>>,
        flags: DialogFlags,
        content_type: &str,
    ) -> AppChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_dialog_new_for_content_type(
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
                content_type.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_app_chooser_dialog_get_heading")]
    #[doc(alias = "get_heading")]
    pub fn heading(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_dialog_get_heading(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_app_chooser_dialog_get_widget")]
    #[doc(alias = "get_widget")]
    pub fn widget(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_dialog_get_widget(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_app_chooser_dialog_set_heading")]
    pub fn set_heading(&self, heading: &str) {
        unsafe {
            ffi::gtk_app_chooser_dialog_set_heading(
                self.to_glib_none().0,
                heading.to_glib_none().0,
            );
        }
    }

    pub fn gfile(&self) -> Option<gio::File> {
        glib::ObjectExt::property(self, "gfile")
    }

    #[doc(alias = "heading")]
    pub fn connect_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_heading_trampoline<F: Fn(&AppChooserDialog) + 'static>(
            this: *mut ffi::GtkAppChooserDialog,
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
                b"notify::heading\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_heading_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for AppChooserDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AppChooserDialog")
    }
}
