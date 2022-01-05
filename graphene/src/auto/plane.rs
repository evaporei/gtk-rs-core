// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Matrix;
use crate::Point3D;
use crate::Vec3;
use glib::translate::*;

glib::wrapper! {
    pub struct Plane(BoxedInline<ffi::graphene_plane_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_plane_get_type(), ptr as *mut _) as *mut ffi::graphene_plane_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_plane_get_type(), ptr as *mut _),
        type_ => || ffi::graphene_plane_get_type(),
    }
}

impl Plane {
    #[doc(alias = "graphene_plane_distance")]
    pub fn distance(&self, point: &Point3D) -> f32 {
        unsafe { ffi::graphene_plane_distance(self.to_glib_none().0, point.to_glib_none().0) }
    }

    #[doc(alias = "graphene_plane_equal")]
    fn equal(&self, b: &Plane) -> bool {
        unsafe { ffi::graphene_plane_equal(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_plane_get_constant")]
    #[doc(alias = "get_constant")]
    pub fn constant(&self) -> f32 {
        unsafe { ffi::graphene_plane_get_constant(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_plane_get_normal")]
    #[doc(alias = "get_normal")]
    pub fn normal(&self) -> Vec3 {
        unsafe {
            let mut normal = Vec3::uninitialized();
            ffi::graphene_plane_get_normal(self.to_glib_none().0, normal.to_glib_none_mut().0);
            normal
        }
    }

    #[doc(alias = "graphene_plane_negate")]
    #[must_use]
    pub fn negate(&self) -> Plane {
        unsafe {
            let mut res = Plane::uninitialized();
            ffi::graphene_plane_negate(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_plane_normalize")]
    #[must_use]
    pub fn normalize(&self) -> Plane {
        unsafe {
            let mut res = Plane::uninitialized();
            ffi::graphene_plane_normalize(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_plane_transform")]
    #[must_use]
    pub fn transform(&self, matrix: &Matrix, normal_matrix: Option<&Matrix>) -> Plane {
        unsafe {
            let mut res = Plane::uninitialized();
            ffi::graphene_plane_transform(
                self.to_glib_none().0,
                matrix.to_glib_none().0,
                normal_matrix.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }
}

impl PartialEq for Plane {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Plane {}
