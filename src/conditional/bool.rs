/// The trait for types that express boolean.
pub trait TypeBool {}

/// True.
pub struct True;

impl TypeBool for True {}

/// False.
pub struct False;

impl TypeBool for False {}

/// The trait for types that have a constant boolean.
/// This is currently used instead of `ConstValue<Type = bool>`.
// TODO: Replace with ConstValue after the ICE is fixed
// https://github.com/rust-lang/rust/issues/108271
pub trait ConstBool {
    const BOOL: bool;
}

/// Convert a constant boolean to the corresponding type.
pub trait ConstBoolToTypeBool {
    type Type: TypeBool;
}

impl<T: ConstBool> ConstBoolToTypeBool for T {
    default type Type = False;
}

impl<T: ConstBool<BOOL = true>> ConstBoolToTypeBool for T {
    type Type = True;
}
