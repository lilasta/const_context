/// The trait used to express a constant value as a type.
/// ```rust
/// use const_context::value::ConstValue;
///
/// struct ConstantZeroU64;
///
/// impl ConstValue for ConstantZeroU64 {
///     type Type = u64;
///     const VALUE: Self::Type = 0;
/// }
///
/// assert_eq!(0u64, ConstantZeroU64::VALUE);
/// ```
pub trait ConstValue {
    /// Type of value.
    type Type;

    /// Constant value.
    const VALUE: Self::Type;
}

/// Evaluate the constant value at compile-time.
#[inline(always)]
pub const fn strict<V: ConstValue>() {
    const { core::mem::forget(V::VALUE) }
}
