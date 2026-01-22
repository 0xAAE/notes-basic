#[inline]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub const fn to_usize(v: f32) -> usize {
    v as usize
}

#[inline]
#[allow(clippy::cast_precision_loss)]
pub const fn to_f32(v: usize) -> f32 {
    v as f32
}
