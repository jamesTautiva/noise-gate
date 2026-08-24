/// Utilities for audio processing optimization

/// Flush denormal numbers to zero to prevent performance degradation
/// Denormals (very small numbers near zero) can cause 100x slowdown on Intel CPUs
#[inline]
pub fn flush_to_zero(x: f32) -> f32 {
    // If absolute value is below denormal threshold, return zero
    // Otherwise return the value unchanged
    const DENORMAL_THRESHOLD: f32 = 1e-20;
    if x.abs() < DENORMAL_THRESHOLD {
        0.0
    } else {
        x
    }
}

/// Alternative implementation using bit manipulation (faster on some architectures)
#[inline]
pub fn flush_to_zero_bitwise(x: f32) -> f32 {
    // This uses the fact that denormals have exponent = 0
    // By setting the exponent to a non-zero value, we force the number to normal
    // or zero
    f32::from_bits(x.to_bits() & 0x7F800000)
}

/// Sanitize audio sample by replacing NaN and Inf with zero
/// This prevents DAWs from auto-muting channels due to invalid values
#[inline]
pub fn sanitize_sample(x: f32) -> f32 {
    if x.is_finite() {
        x
    } else {
        0.0
    }
}
