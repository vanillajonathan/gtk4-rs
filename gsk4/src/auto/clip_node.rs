// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct ClipNode(Object<ffi::GskClipNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_clip_node_get_type(),
    }
}

impl ClipNode {
    #[doc(alias = "gsk_clip_node_new")]
    pub fn new<P: IsA<RenderNode>>(child: &P, clip: &graphene::Rect) -> ClipNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_clip_node_new(
                child.as_ref().to_glib_none().0,
                clip.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_clip_node_get_child")]
    pub fn get_child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_clip_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_clip_node_get_clip")]
    pub fn get_clip(&self) -> Option<graphene::Rect> {
        unsafe { from_glib_none(ffi::gsk_clip_node_get_clip(self.to_glib_none().0)) }
    }
}

impl fmt::Display for ClipNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ClipNode")
    }
}
