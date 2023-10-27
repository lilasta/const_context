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
    type Type;
    const VALUE: Self::Type;
}
