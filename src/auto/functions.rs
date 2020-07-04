// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_x11_sys;
use glib::object::IsA;
use glib::translate::*;
use xlib;
use X11DeviceCore;
use X11DeviceManagerCore;
use X11Display;
use X11Window;

//pub fn x11_atom_to_xatom(atom: /*Ignored*/&gdk::Atom) -> /*Ignored*/xlib::Atom {
//    unsafe { TODO: call gdk_x11_sys:gdk_x11_atom_to_xatom() }
//}

//pub fn x11_atom_to_xatom_for_display(display: &X11Display, atom: /*Ignored*/&gdk::Atom) -> /*Ignored*/xlib::Atom {
//    unsafe { TODO: call gdk_x11_sys:gdk_x11_atom_to_xatom_for_display() }
//}

pub fn x11_device_get_id(device: &X11DeviceCore) -> i32 {
    skip_assert_initialized!();
    unsafe { gdk_x11_sys::gdk_x11_device_get_id(device.to_glib_none().0) }
}

pub fn x11_device_manager_lookup<P: IsA<X11DeviceManagerCore>>(
    device_manager: &P,
    device_id: i32,
) -> Option<X11DeviceCore> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(gdk_x11_sys::gdk_x11_device_manager_lookup(
            device_manager.as_ref().to_glib_none().0,
            device_id,
        ))
    }
}

pub fn x11_free_compound_text(ctext: u8) {
    assert_initialized_main_thread!();
    unsafe {
        gdk_x11_sys::gdk_x11_free_compound_text(ctext);
    }
}

pub fn x11_free_text_list(list: &str) {
    assert_initialized_main_thread!();
    unsafe {
        gdk_x11_sys::gdk_x11_free_text_list(list.to_glib_none().0);
    }
}

pub fn x11_get_default_root_xwindow() -> xlib::Window {
    assert_initialized_main_thread!();
    unsafe { gdk_x11_sys::gdk_x11_get_default_root_xwindow() }
}

pub fn x11_get_default_screen() -> i32 {
    assert_initialized_main_thread!();
    unsafe { gdk_x11_sys::gdk_x11_get_default_screen() }
}

//pub fn x11_get_default_xdisplay() -> /*Ignored*/Option<xlib::Display> {
//    unsafe { TODO: call gdk_x11_sys:gdk_x11_get_default_xdisplay() }
//}

//#[cfg_attr(feature = "v3_24", deprecated)]
//#[cfg(any(feature = "v3_24_2", feature = "dox"))]
//pub fn x11_get_parent_relative_pattern() -> /*Ignored*/Option<cairo::Pattern> {
//    unsafe { TODO: call gdk_x11_sys:gdk_x11_get_parent_relative_pattern() }
//}

pub fn x11_get_server_time(window: &X11Window) -> u32 {
    skip_assert_initialized!();
    unsafe { gdk_x11_sys::gdk_x11_get_server_time(window.to_glib_none().0) }
}

//pub fn x11_get_xatom_by_name(atom_name: &str) -> /*Ignored*/xlib::Atom {
//    unsafe { TODO: call gdk_x11_sys:gdk_x11_get_xatom_by_name() }
//}

//pub fn x11_get_xatom_by_name_for_display(display: &X11Display, atom_name: &str) -> /*Ignored*/xlib::Atom {
//    unsafe { TODO: call gdk_x11_sys:gdk_x11_get_xatom_by_name_for_display() }
//}

//pub fn x11_get_xatom_name(xatom: /*Ignored*/xlib::Atom) -> Option<GString> {
//    unsafe { TODO: call gdk_x11_sys:gdk_x11_get_xatom_name() }
//}

//pub fn x11_get_xatom_name_for_display(display: &X11Display, xatom: /*Ignored*/xlib::Atom) -> Option<GString> {
//    unsafe { TODO: call gdk_x11_sys:gdk_x11_get_xatom_name_for_display() }
//}

pub fn x11_grab_server() {
    assert_initialized_main_thread!();
    unsafe {
        gdk_x11_sys::gdk_x11_grab_server();
    }
}

//pub fn x11_lookup_xdisplay(xdisplay: /*Ignored*/&mut xlib::Display) -> Option<X11Display> {
//    unsafe { TODO: call gdk_x11_sys:gdk_x11_lookup_xdisplay() }
//}

pub fn x11_register_standard_event_type(display: &X11Display, event_base: i32, n_events: i32) {
    skip_assert_initialized!();
    unsafe {
        gdk_x11_sys::gdk_x11_register_standard_event_type(
            display.to_glib_none().0,
            event_base,
            n_events,
        );
    }
}

pub fn x11_set_sm_client_id(sm_client_id: Option<&str>) {
    assert_initialized_main_thread!();
    unsafe {
        gdk_x11_sys::gdk_x11_set_sm_client_id(sm_client_id.to_glib_none().0);
    }
}

pub fn x11_ungrab_server() {
    assert_initialized_main_thread!();
    unsafe {
        gdk_x11_sys::gdk_x11_ungrab_server();
    }
}

//pub fn x11_xatom_to_atom(xatom: /*Ignored*/xlib::Atom) -> /*Ignored*/Option<gdk::Atom> {
//    unsafe { TODO: call gdk_x11_sys:gdk_x11_xatom_to_atom() }
//}

//pub fn x11_xatom_to_atom_for_display(display: &X11Display, xatom: /*Ignored*/xlib::Atom) -> /*Ignored*/Option<gdk::Atom> {
//    unsafe { TODO: call gdk_x11_sys:gdk_x11_xatom_to_atom_for_display() }
//}