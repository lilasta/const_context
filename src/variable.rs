pub mod list;

/// Variables for binding constant values.
/// ```rust
/// use const_context::ctx;
///
/// struct Zero;
///
/// ctx! {
///     // Zero: u64 = 0
///     set (Zero, u64) = 0;
/// };
/// ```
pub trait Variable {
    /// Variable name.
    type Name: 'static;

    /// Variable type.
    type Type: 'static;
}

/// `_: ()` is expressed as `()`.
impl Variable for () {
    type Name = ();
    type Type = ();
}

/// `name: type` is expressed as `(Name, Type)`.
impl<Name, Type> Variable for (Name, Type)
where
    Name: 'static,
    Type: 'static,
{
    type Name = Name;
    type Type = Type;
}
