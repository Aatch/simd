use super::*;
use super::bitcast;

//pub use super::{u64x2, i64x2, f64x2, bool64ix2, bool64fx2};

extern "platform-intrinsic" {
    fn x86_mm_sqrt_ps(x: f32x4) -> f32x4;

    fn x86_mm_rsqrt_ps(x: f32x4) -> f32x4;
    fn x86_mm_rcp_ps(x: f32x4) -> f32x4;

    fn x86_mm_max_ps(x: f32x4, y: f32x4) -> f32x4;

    fn x86_mm_min_ps(x: f32x4, y: f32x4) -> f32x4;

    fn x86_mm_movemask_ps(x: f32x4) -> i32;
    fn x86_mm_movemask_epi8(x: u8x16) -> i32;
}

impl f32x4 {
    #[inline]
    pub fn sqrt(self) -> f32x4 {
        unsafe {x86_mm_sqrt_ps(self)}
    }
    #[inline]
    pub fn approx_rsqrt(self) -> f32x4 {
        unsafe {x86_mm_rsqrt_ps(self)}
    }
    #[inline]
    pub fn approx_reciprocal(self) -> f32x4 {
        unsafe {x86_mm_rcp_ps(self)}
    }
    #[inline]
    pub fn max(self, other: f32x4) -> f32x4 {
        unsafe {x86_mm_max_ps(self, other)}
    }
    #[inline]
    pub fn min(self, other: f32x4) -> f32x4 {
        unsafe {x86_mm_min_ps(self, other)}
    }
}

macro_rules! bool_impls {
    ($($ty: ty, $movemask: ident, $width: expr;)*) => {
        $(impl $ty {
            #[inline]
            pub fn any(self) -> bool {
                unsafe {$movemask(bitcast(self)) != 0}
            }
            #[inline]
            pub fn all(self) -> bool {
                unsafe {$movemask(bitcast(self)) == (1 << $width) - 1}
            }
        })*
    }
}
bool_impls! {
    bool32fx4, x86_mm_movemask_ps, 4;

    bool8ix16, x86_mm_movemask_epi8, 16;
    bool16ix8, x86_mm_movemask_epi8, 16;
    bool32ix4, x86_mm_movemask_epi8, 16;
}