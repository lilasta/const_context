use core::marker::PhantomData;

use crate::utils::{is_same_type, str_concat_ct};
use crate::value::bool::ConstBool;
use crate::value::ConstValue;
use crate::variable::Variable;

/// List of variable.
pub trait VariableList {
    /// Type of list. (Used to find a variable at compile-time)
    const KIND: VariableListKind;

    /// Variable.
    type Var: Variable;

    /// Value of the variable.
    const VALUE: <Self::Var as Variable>::Type;

    /// The rest of the list.
    type Rest: VariableList;

    type IfThen: VariableList;
    type IfElse: VariableList;
}

/// Type of list. (Used to find a variable at compile-time)
pub enum VariableListKind {
    Empty,
    Cons,
    Remove,
    If(bool),
}

/// Empty list.
pub struct VariableListEmpty;

/// Dummy type to express variable name and variable type for VariableListEmpty.
/// This type must not be used by the user.
#[doc(hidden)]
pub struct __VariableListEmptyDummy;

impl VariableList for VariableListEmpty {
    const KIND: VariableListKind = VariableListKind::Empty;

    type Var = (__VariableListEmptyDummy, __VariableListEmptyDummy);
    const VALUE: __VariableListEmptyDummy = unreachable!();

    type Rest = VariableListEmpty;
    type IfThen = VariableListEmpty;
    type IfElse = VariableListEmpty;
}

/// List that has a value.
pub struct VariableListCons<Var, Value, Rest>(PhantomData<(Var, Value, Rest)>);

impl<Var, Value, Rest> VariableList for VariableListCons<Var, Value, Rest>
where
    Var: Variable,
    Value: ConstValue<Type = Var::Type>,
    Rest: VariableList,
{
    const KIND: VariableListKind = VariableListKind::Cons;

    type Var = Var;
    const VALUE: Var::Type = Value::VALUE;

    type Rest = Rest;
    type IfThen = VariableListEmpty;
    type IfElse = VariableListEmpty;
}

pub struct VariableListIf<Bool, Then, Else>(PhantomData<(Bool, Then, Else)>);

/// Dummy type to express variable name and variable type for VariableListIf.
/// This type must not be used by the user.
#[doc(hidden)]
pub struct __VariableListIfDummy;

impl<Bool, Then, Else> VariableList for VariableListIf<Bool, Then, Else>
where
    Bool: ConstBool,
    Then: VariableList,
    Else: VariableList,
{
    const KIND: VariableListKind = VariableListKind::If(Bool::VALUE);

    type Var = (__VariableListIfDummy, __VariableListIfDummy);
    const VALUE: __VariableListIfDummy = __VariableListIfDummy;

    type Rest = VariableListEmpty;
    type IfThen = Then;
    type IfElse = Else;
}

/// This struct removes the variable value from `List``.
pub struct VariableListRemove<Var, List>(PhantomData<(Var, List)>);

impl<Var, List> VariableList for VariableListRemove<Var, List>
where
    Var: Variable,
    List: VariableList,
{
    const KIND: VariableListKind = VariableListKind::Remove;

    type Var = Var;
    const VALUE: Var::Type = unreachable!();

    type Rest = List;
    type IfThen = VariableListEmpty;
    type IfElse = VariableListEmpty;
}

/// Finds the given variable in the list and returns its value.
#[track_caller]
pub const fn find_variable<List, Var>() -> Var::Type
where
    List: VariableList,
    Var: Variable,
{
    let is_name_same = is_same_type::<Var::Name, <List::Var as Variable>::Name>();
    let is_type_same = is_same_type::<Var::Type, <List::Var as Variable>::Type>();

    match List::KIND {
        VariableListKind::Empty => panic!("{}", error_not_found::<Var::Name>()),
        VariableListKind::Remove if is_name_same => panic!("{}", error_not_found::<Var::Name>()),
        VariableListKind::Cons if is_name_same => {
            assert!(
                is_type_same,
                "{}",
                error_unexpected_type::<Var::Type, <List::Var as Variable>::Type>()
            );

            // Using transmute_unchecked instead of transmute to avoid the "cannot transmute between types" error.
            // SAFETY: Already checked that two types are the same with `is_type_same`.
            unsafe { core::intrinsics::transmute_unchecked(List::VALUE) }
        }
        VariableListKind::If(true) => find_variable::<List::IfThen, Var>(),
        VariableListKind::If(false) => find_variable::<List::IfElse, Var>(),
        _ => find_variable::<List::Rest, Var>(),
    }
}

/// Returns whether or not the list contains the given variable.
#[track_caller]
pub const fn is_variable_in<List, Var>() -> bool
where
    List: VariableList,
    Var: Variable,
{
    let is_name_same = is_same_type::<Var::Name, <List::Var as Variable>::Name>();
    let is_type_same = is_same_type::<Var::Type, <List::Var as Variable>::Type>();

    match List::KIND {
        VariableListKind::Empty => false,
        VariableListKind::Remove if is_name_same => false,
        VariableListKind::Cons if is_name_same => {
            assert!(
                is_type_same,
                "{}",
                error_unexpected_type::<Var::Type, <List::Var as Variable>::Type>()
            );
            true
        }
        VariableListKind::If(true) => is_variable_in::<List::IfThen, Var>(),
        VariableListKind::If(false) => is_variable_in::<List::IfElse, Var>(),
        _ => is_variable_in::<List::Rest, Var>(),
    }
}

const fn error_not_found<Key>() -> &'static str {
    let type_name = core::any::type_name::<Key>();
    str_concat_ct(
        str_concat_ct("The key `", type_name),
        "` is not found in current context.",
    )
}

const fn error_unexpected_type<Expected, Value>() -> &'static str {
    let type_name_expect = core::any::type_name::<Expected>();
    let type_name_value = core::any::type_name::<Value>();
    str_concat_ct(
        str_concat_ct("Mismatched types: expected `", type_name_expect),
        str_concat_ct("`, found `", str_concat_ct(type_name_value, "`.")),
    )
}
