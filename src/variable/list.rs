use core::marker::PhantomData;

use crate::utils::{is_same_type, str_concat_ct};
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
}

/// Type of list. (Used to find a variable at compile-time)
pub enum VariableListKind {
    Empty,
    Cons,
    Remove,
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
}

/// This struct removes the variable value from `List``.
pub struct VariableListRemove<Var, List>(PhantomData<(Var, List)>);

/// Dummy type to express variable name and variable type for VariableListRemove.
/// This type must not be used by the user.
#[doc(hidden)]
pub struct __VariableListRemove;

impl<Var, List> VariableList for VariableListRemove<Var, List>
where
    Var: Variable,
    List: VariableList,
{
    const KIND: VariableListKind = VariableListKind::Remove;

    type Var = (__VariableListRemove, __VariableListRemove);
    const VALUE: __VariableListRemove = unreachable!();

    type Rest = List;
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

            let src = List::VALUE;

            // Cast.
            // Using transmute_copy instead of transmute to avoid the "cannot transmute between types" error.
            // SAFETY: Already checked that two types are the same with `is_type_same`.
            let value = unsafe { core::mem::transmute_copy(&src) };

            // Don't drop src
            core::mem::forget(src);

            value
        }
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
