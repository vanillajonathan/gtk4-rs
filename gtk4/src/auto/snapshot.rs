// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::StyleContext;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkSnapshot")]
    pub struct Snapshot(Object<ffi::GtkSnapshot, ffi::GtkSnapshotClass>) @extends gdk::Snapshot;

    match fn {
        type_ => || ffi::gtk_snapshot_get_type(),
    }
}

impl Snapshot {
    #[doc(alias = "gtk_snapshot_new")]
    pub fn new() -> Snapshot {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_snapshot_new()) }
    }

    #[doc(alias = "gtk_snapshot_append_cairo")]
    pub fn append_cairo(&self, bounds: &graphene::Rect) -> cairo::Context {
        unsafe {
            from_glib_full(ffi::gtk_snapshot_append_cairo(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_snapshot_append_color")]
    pub fn append_color(&self, color: &gdk::RGBA, bounds: &graphene::Rect) {
        unsafe {
            ffi::gtk_snapshot_append_color(
                self.to_glib_none().0,
                color.to_glib_none().0,
                bounds.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_conic_gradient")]
    pub fn append_conic_gradient(
        &self,
        bounds: &graphene::Rect,
        center: &graphene::Point,
        rotation: f32,
        stops: &[gsk::ColorStop],
    ) {
        let n_stops = stops.len() as usize;
        unsafe {
            ffi::gtk_snapshot_append_conic_gradient(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                rotation,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_inset_shadow")]
    pub fn append_inset_shadow(
        &self,
        outline: &gsk::RoundedRect,
        color: &gdk::RGBA,
        dx: f32,
        dy: f32,
        spread: f32,
        blur_radius: f32,
    ) {
        unsafe {
            ffi::gtk_snapshot_append_inset_shadow(
                self.to_glib_none().0,
                outline.to_glib_none().0,
                color.to_glib_none().0,
                dx,
                dy,
                spread,
                blur_radius,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_layout")]
    pub fn append_layout(&self, layout: &pango::Layout, color: &gdk::RGBA) {
        unsafe {
            ffi::gtk_snapshot_append_layout(
                self.to_glib_none().0,
                layout.to_glib_none().0,
                color.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_linear_gradient")]
    pub fn append_linear_gradient(
        &self,
        bounds: &graphene::Rect,
        start_point: &graphene::Point,
        end_point: &graphene::Point,
        stops: &[gsk::ColorStop],
    ) {
        let n_stops = stops.len() as usize;
        unsafe {
            ffi::gtk_snapshot_append_linear_gradient(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
                start_point.to_glib_none().0,
                end_point.to_glib_none().0,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_node")]
    pub fn append_node(&self, node: impl AsRef<gsk::RenderNode>) {
        unsafe {
            ffi::gtk_snapshot_append_node(self.to_glib_none().0, node.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_append_outset_shadow")]
    pub fn append_outset_shadow(
        &self,
        outline: &gsk::RoundedRect,
        color: &gdk::RGBA,
        dx: f32,
        dy: f32,
        spread: f32,
        blur_radius: f32,
    ) {
        unsafe {
            ffi::gtk_snapshot_append_outset_shadow(
                self.to_glib_none().0,
                outline.to_glib_none().0,
                color.to_glib_none().0,
                dx,
                dy,
                spread,
                blur_radius,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_radial_gradient")]
    pub fn append_radial_gradient(
        &self,
        bounds: &graphene::Rect,
        center: &graphene::Point,
        hradius: f32,
        vradius: f32,
        start: f32,
        end: f32,
        stops: &[gsk::ColorStop],
    ) {
        let n_stops = stops.len() as usize;
        unsafe {
            ffi::gtk_snapshot_append_radial_gradient(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                hradius,
                vradius,
                start,
                end,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_repeating_linear_gradient")]
    pub fn append_repeating_linear_gradient(
        &self,
        bounds: &graphene::Rect,
        start_point: &graphene::Point,
        end_point: &graphene::Point,
        stops: &[gsk::ColorStop],
    ) {
        let n_stops = stops.len() as usize;
        unsafe {
            ffi::gtk_snapshot_append_repeating_linear_gradient(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
                start_point.to_glib_none().0,
                end_point.to_glib_none().0,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_repeating_radial_gradient")]
    pub fn append_repeating_radial_gradient(
        &self,
        bounds: &graphene::Rect,
        center: &graphene::Point,
        hradius: f32,
        vradius: f32,
        start: f32,
        end: f32,
        stops: &[gsk::ColorStop],
    ) {
        let n_stops = stops.len() as usize;
        unsafe {
            ffi::gtk_snapshot_append_repeating_radial_gradient(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                hradius,
                vradius,
                start,
                end,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_texture")]
    pub fn append_texture(&self, texture: &impl IsA<gdk::Texture>, bounds: &graphene::Rect) {
        unsafe {
            ffi::gtk_snapshot_append_texture(
                self.to_glib_none().0,
                texture.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_gl_shader_pop_texture")]
    pub fn gl_shader_pop_texture(&self) {
        unsafe {
            ffi::gtk_snapshot_gl_shader_pop_texture(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_perspective")]
    pub fn perspective(&self, depth: f32) {
        unsafe {
            ffi::gtk_snapshot_perspective(self.to_glib_none().0, depth);
        }
    }

    #[doc(alias = "gtk_snapshot_pop")]
    pub fn pop(&self) {
        unsafe {
            ffi::gtk_snapshot_pop(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_push_blend")]
    pub fn push_blend(&self, blend_mode: gsk::BlendMode) {
        unsafe {
            ffi::gtk_snapshot_push_blend(self.to_glib_none().0, blend_mode.into_glib());
        }
    }

    #[doc(alias = "gtk_snapshot_push_blur")]
    pub fn push_blur(&self, radius: f64) {
        unsafe {
            ffi::gtk_snapshot_push_blur(self.to_glib_none().0, radius);
        }
    }

    #[doc(alias = "gtk_snapshot_push_clip")]
    pub fn push_clip(&self, bounds: &graphene::Rect) {
        unsafe {
            ffi::gtk_snapshot_push_clip(self.to_glib_none().0, bounds.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_push_color_matrix")]
    pub fn push_color_matrix(
        &self,
        color_matrix: &graphene::Matrix,
        color_offset: &graphene::Vec4,
    ) {
        unsafe {
            ffi::gtk_snapshot_push_color_matrix(
                self.to_glib_none().0,
                color_matrix.to_glib_none().0,
                color_offset.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_push_cross_fade")]
    pub fn push_cross_fade(&self, progress: f64) {
        unsafe {
            ffi::gtk_snapshot_push_cross_fade(self.to_glib_none().0, progress);
        }
    }

    #[doc(alias = "gtk_snapshot_push_gl_shader")]
    pub fn push_gl_shader(
        &self,
        shader: &gsk::GLShader,
        bounds: &graphene::Rect,
        take_args: &glib::Bytes,
    ) {
        unsafe {
            ffi::gtk_snapshot_push_gl_shader(
                self.to_glib_none().0,
                shader.to_glib_none().0,
                bounds.to_glib_none().0,
                take_args.to_glib_full(),
            );
        }
    }

    #[doc(alias = "gtk_snapshot_push_opacity")]
    pub fn push_opacity(&self, opacity: f64) {
        unsafe {
            ffi::gtk_snapshot_push_opacity(self.to_glib_none().0, opacity);
        }
    }

    #[doc(alias = "gtk_snapshot_push_repeat")]
    pub fn push_repeat(&self, bounds: &graphene::Rect, child_bounds: Option<&graphene::Rect>) {
        unsafe {
            ffi::gtk_snapshot_push_repeat(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
                child_bounds.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_push_rounded_clip")]
    pub fn push_rounded_clip(&self, bounds: &gsk::RoundedRect) {
        unsafe {
            ffi::gtk_snapshot_push_rounded_clip(self.to_glib_none().0, bounds.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_push_shadow")]
    pub fn push_shadow(&self, shadow: &[gsk::Shadow]) {
        let n_shadows = shadow.len() as usize;
        unsafe {
            ffi::gtk_snapshot_push_shadow(
                self.to_glib_none().0,
                shadow.to_glib_none().0,
                n_shadows,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_render_background")]
    pub fn render_background(
        &self,
        context: &impl IsA<StyleContext>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_background(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_render_focus")]
    pub fn render_focus(
        &self,
        context: &impl IsA<StyleContext>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_focus(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_render_frame")]
    pub fn render_frame(
        &self,
        context: &impl IsA<StyleContext>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_frame(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_render_insertion_cursor")]
    pub fn render_insertion_cursor(
        &self,
        context: &impl IsA<StyleContext>,
        x: f64,
        y: f64,
        layout: &pango::Layout,
        index: i32,
        direction: pango::Direction,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_insertion_cursor(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                layout.to_glib_none().0,
                index,
                direction.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_snapshot_render_layout")]
    pub fn render_layout(
        &self,
        context: &impl IsA<StyleContext>,
        x: f64,
        y: f64,
        layout: &pango::Layout,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_layout(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                layout.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_restore")]
    pub fn restore(&self) {
        unsafe {
            ffi::gtk_snapshot_restore(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_rotate")]
    pub fn rotate(&self, angle: f32) {
        unsafe {
            ffi::gtk_snapshot_rotate(self.to_glib_none().0, angle);
        }
    }

    #[doc(alias = "gtk_snapshot_rotate_3d")]
    pub fn rotate_3d(&self, angle: f32, axis: &graphene::Vec3) {
        unsafe {
            ffi::gtk_snapshot_rotate_3d(self.to_glib_none().0, angle, axis.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_save")]
    pub fn save(&self) {
        unsafe {
            ffi::gtk_snapshot_save(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_scale")]
    pub fn scale(&self, factor_x: f32, factor_y: f32) {
        unsafe {
            ffi::gtk_snapshot_scale(self.to_glib_none().0, factor_x, factor_y);
        }
    }

    #[doc(alias = "gtk_snapshot_scale_3d")]
    pub fn scale_3d(&self, factor_x: f32, factor_y: f32, factor_z: f32) {
        unsafe {
            ffi::gtk_snapshot_scale_3d(self.to_glib_none().0, factor_x, factor_y, factor_z);
        }
    }

    #[doc(alias = "gtk_snapshot_to_node")]
    pub fn to_node(&self) -> gsk::RenderNode {
        unsafe { from_glib_full(ffi::gtk_snapshot_to_node(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_snapshot_to_paintable")]
    pub fn to_paintable(&self, size: Option<&graphene::Size>) -> gdk::Paintable {
        unsafe {
            from_glib_full(ffi::gtk_snapshot_to_paintable(
                self.to_glib_none().0,
                size.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_snapshot_transform")]
    pub fn transform(&self, transform: Option<&gsk::Transform>) {
        unsafe {
            ffi::gtk_snapshot_transform(self.to_glib_none().0, transform.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_transform_matrix")]
    pub fn transform_matrix(&self, matrix: &graphene::Matrix) {
        unsafe {
            ffi::gtk_snapshot_transform_matrix(self.to_glib_none().0, matrix.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_translate")]
    pub fn translate(&self, point: &graphene::Point) {
        unsafe {
            ffi::gtk_snapshot_translate(self.to_glib_none().0, point.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_translate_3d")]
    pub fn translate_3d(&self, point: &graphene::Point3D) {
        unsafe {
            ffi::gtk_snapshot_translate_3d(self.to_glib_none().0, point.to_glib_none().0);
        }
    }
}

impl Default for Snapshot {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Snapshot")
    }
}
