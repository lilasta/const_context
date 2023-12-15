pub mod bool;

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
pub const fn strict<Value: ConstValue>() {
    const { core::mem::forget(Value::VALUE) }
}

/// Evaluate the constant value at compile-time if `Bool` is true.
#[inline(always)]
pub const fn strict_if<Bool: ConstValue<Type = bool>, Value: ConstValue>() {
    const {
        if Bool::VALUE {
            strict::<Value>();
        }
    }
}
