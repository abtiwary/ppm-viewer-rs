// Generated by swizzlegen. Do not edit.

use super::Vec3Swizzles;
use crate::{Vec2, Vec3A, Vec4, XY};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

impl Vec3Swizzles for Vec3A {
    type Vec2 = Vec2;
    type Vec4 = Vec4;

    #[inline]
    fn xxxx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_00)) }
    }
    #[inline]
    fn xxxy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_00_00)) }
    }
    #[inline]
    fn xxxz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_00_00)) }
    }
    #[inline]
    fn xxyx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_00_00)) }
    }
    #[inline]
    fn xxyy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_00_00)) }
    }
    #[inline]
    fn xxyz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_00_00)) }
    }
    #[inline]
    fn xxzx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_00_00)) }
    }
    #[inline]
    fn xxzy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_00_00)) }
    }
    #[inline]
    fn xxzz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_00_00)) }
    }
    #[inline]
    fn xyxx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_00)) }
    }
    #[inline]
    fn xyxy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_01_00)) }
    }
    #[inline]
    fn xyxz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_01_00)) }
    }
    #[inline]
    fn xyyx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_01_00)) }
    }
    #[inline]
    fn xyyy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_00)) }
    }
    #[inline]
    fn xyyz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_01_00)) }
    }
    #[inline]
    fn xyzx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_01_00)) }
    }
    #[inline]
    fn xyzy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_01_00)) }
    }
    #[inline]
    fn xyzz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_01_00)) }
    }
    #[inline]
    fn xzxx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_00)) }
    }
    #[inline]
    fn xzxy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_10_00)) }
    }
    #[inline]
    fn xzxz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_10_00)) }
    }
    #[inline]
    fn xzyx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_10_00)) }
    }
    #[inline]
    fn xzyy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_10_00)) }
    }
    #[inline]
    fn xzyz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_10_00)) }
    }
    #[inline]
    fn xzzx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_10_00)) }
    }
    #[inline]
    fn xzzy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_10_00)) }
    }
    #[inline]
    fn xzzz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_00)) }
    }
    #[inline]
    fn yxxx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_01)) }
    }
    #[inline]
    fn yxxy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_00_01)) }
    }
    #[inline]
    fn yxxz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_00_01)) }
    }
    #[inline]
    fn yxyx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_00_01)) }
    }
    #[inline]
    fn yxyy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_00_01)) }
    }
    #[inline]
    fn yxyz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_00_01)) }
    }
    #[inline]
    fn yxzx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_00_01)) }
    }
    #[inline]
    fn yxzy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_00_01)) }
    }
    #[inline]
    fn yxzz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_00_01)) }
    }
    #[inline]
    fn yyxx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_01)) }
    }
    #[inline]
    fn yyxy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_01_01)) }
    }
    #[inline]
    fn yyxz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_01_01)) }
    }
    #[inline]
    fn yyyx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_01_01)) }
    }
    #[inline]
    fn yyyy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01)) }
    }
    #[inline]
    fn yyyz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_01_01)) }
    }
    #[inline]
    fn yyzx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_01_01)) }
    }
    #[inline]
    fn yyzy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_01_01)) }
    }
    #[inline]
    fn yyzz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_01_01)) }
    }
    #[inline]
    fn yzxx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_01)) }
    }
    #[inline]
    fn yzxy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_10_01)) }
    }
    #[inline]
    fn yzxz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_10_01)) }
    }
    #[inline]
    fn yzyx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_10_01)) }
    }
    #[inline]
    fn yzyy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_10_01)) }
    }
    #[inline]
    fn yzyz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_10_01)) }
    }
    #[inline]
    fn yzzx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_10_01)) }
    }
    #[inline]
    fn yzzy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_10_01)) }
    }
    #[inline]
    fn yzzz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_01)) }
    }
    #[inline]
    fn zxxx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_10)) }
    }
    #[inline]
    fn zxxy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_00_10)) }
    }
    #[inline]
    fn zxxz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_00_10)) }
    }
    #[inline]
    fn zxyx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_00_10)) }
    }
    #[inline]
    fn zxyy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_00_10)) }
    }
    #[inline]
    fn zxyz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_00_10)) }
    }
    #[inline]
    fn zxzx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_00_10)) }
    }
    #[inline]
    fn zxzy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_00_10)) }
    }
    #[inline]
    fn zxzz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_00_10)) }
    }
    #[inline]
    fn zyxx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_10)) }
    }
    #[inline]
    fn zyxy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_01_10)) }
    }
    #[inline]
    fn zyxz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_01_10)) }
    }
    #[inline]
    fn zyyx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_01_10)) }
    }
    #[inline]
    fn zyyy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_10)) }
    }
    #[inline]
    fn zyyz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_01_10)) }
    }
    #[inline]
    fn zyzx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_01_10)) }
    }
    #[inline]
    fn zyzy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_01_10)) }
    }
    #[inline]
    fn zyzz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_01_10)) }
    }
    #[inline]
    fn zzxx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_10)) }
    }
    #[inline]
    fn zzxy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_10_10)) }
    }
    #[inline]
    fn zzxz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_10_10)) }
    }
    #[inline]
    fn zzyx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_10_10)) }
    }
    #[inline]
    fn zzyy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_10_10)) }
    }
    #[inline]
    fn zzyz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_10_10)) }
    }
    #[inline]
    fn zzzx(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_10_10)) }
    }
    #[inline]
    fn zzzy(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_10_10)) }
    }
    #[inline]
    fn zzzz(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10)) }
    }
    #[inline]
    fn xxx(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_00)) }
    }
    #[inline]
    fn xxy(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_01_00_00)) }
    }
    #[inline]
    fn xxz(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_10_00_00)) }
    }
    #[inline]
    fn xyx(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_00)) }
    }
    #[inline]
    fn xyy(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_01_01_00)) }
    }
    #[inline]
    fn xzx(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_00)) }
    }
    #[inline]
    fn xzy(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_01_10_00)) }
    }
    #[inline]
    fn xzz(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_10_10_00)) }
    }
    #[inline]
    fn yxx(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_01)) }
    }
    #[inline]
    fn yxy(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_01_00_01)) }
    }
    #[inline]
    fn yxz(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_10_00_01)) }
    }
    #[inline]
    fn yyx(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_01)) }
    }
    #[inline]
    fn yyy(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_01_01_01)) }
    }
    #[inline]
    fn yyz(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_10_01_01)) }
    }
    #[inline]
    fn yzx(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_01)) }
    }
    #[inline]
    fn yzy(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_01_10_01)) }
    }
    #[inline]
    fn yzz(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_10_10_01)) }
    }
    #[inline]
    fn zxx(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_10)) }
    }
    #[inline]
    fn zxy(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_01_00_10)) }
    }
    #[inline]
    fn zxz(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_10_00_10)) }
    }
    #[inline]
    fn zyx(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_10)) }
    }
    #[inline]
    fn zyy(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_01_01_10)) }
    }
    #[inline]
    fn zyz(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_10_01_10)) }
    }
    #[inline]
    fn zzx(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_10)) }
    }
    #[inline]
    fn zzy(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_01_10_10)) }
    }
    #[inline]
    fn zzz(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_10_10_10)) }
    }
    #[inline]
    fn xx(self) -> Vec2 {
        unsafe { Vec2(XY::from(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_00))) }
    }
    #[inline]
    fn xy(self) -> Vec2 {
        unsafe { Vec2(XY::from(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_00))) }
    }
    #[inline]
    fn xz(self) -> Vec2 {
        unsafe { Vec2(XY::from(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_00))) }
    }
    #[inline]
    fn yx(self) -> Vec2 {
        unsafe { Vec2(XY::from(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_01))) }
    }
    #[inline]
    fn yy(self) -> Vec2 {
        unsafe { Vec2(XY::from(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_01))) }
    }
    #[inline]
    fn yz(self) -> Vec2 {
        unsafe { Vec2(XY::from(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_01))) }
    }
    #[inline]
    fn zx(self) -> Vec2 {
        unsafe { Vec2(XY::from(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_10))) }
    }
    #[inline]
    fn zy(self) -> Vec2 {
        unsafe { Vec2(XY::from(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_10))) }
    }
    #[inline]
    fn zz(self) -> Vec2 {
        unsafe { Vec2(XY::from(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_10))) }
    }
}
