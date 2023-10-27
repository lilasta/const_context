use core::marker::{Destruct, PhantomData};

use crate::utils::{str_concat, type_eq};
use crate::value::ConstValue;

pub trait ConstVariable {
    type Key: 'static;
    type Value: 'static + ~const Destruct;
}

impl ConstVariable for () {
    type Key = ();
    type Value = ();
}

impl<K, V> ConstVariable for (K, V)
where
    K: 'static,
    V: 'static,
{
    type Key = K;
    type Value = V;
}

pub struct VariableListEnd;

pub struct VariableListHas<Var, Value: ConstValue, Next>(PhantomData<(Var, Value, Next)>);

pub struct VariableListRemoved<Var, Next>(PhantomData<(Var, Next)>);

pub enum VariableListType {
    End,
    Has,
    Removed,
}

pub trait VariableList {
    type Next: VariableList;
    type Variable: ConstVariable;
    const TYPE: VariableListType;
    const VALUE: <Self::Variable as ConstVariable>::Value;
}

impl VariableList for VariableListEnd {
    type Next = VariableListEnd;
    type Variable = ();
    const TYPE: VariableListType = VariableListType::End;
    const VALUE: <Self::Variable as ConstVariable>::Value = panic!("");
}

impl<Var, Value, Next> VariableList for VariableListHas<Var, Value, Next>
where
    Var: ConstVariable,
    Value: ConstValue<Type = Var::Value>,
    Next: VariableList,
{
    type Next = Next;
    type Variable = Var;
    const TYPE: VariableListType = VariableListType::Has;
    const VALUE: Var::Value = Value::VALUE;
}

impl<Var, Next> VariableList for VariableListRemoved<Var, Next>
where
    Var: ConstVariable,
    Next: VariableList,
{
    type Next = Next;
    type Variable = Var;
    const TYPE: VariableListType = VariableListType::Removed;
    const VALUE: Var::Value = panic!();
}

const fn error_not_found<Key>() -> &'static str {
    let type_name = core::any::type_name::<Key>();
    str_concat(
        str_concat("The key `", type_name),
        "` is not found in current context.",
    )
}

const fn error_unexpected_type<Expected, Value>() -> &'static str {
    let type_name_expect = core::any::type_name::<Expected>();
    let type_name_value = core::any::type_name::<Value>();
    str_concat(
        str_concat("Mismatched types: expected `", type_name_expect),
        str_concat("`, found `", str_concat(type_name_value, "`.")),
    )
}

#[track_caller]
pub const fn find_variable<List, Var>() -> Var::Value
where
    List: VariableList,
    Var: ConstVariable,
{
    match List::TYPE {
        VariableListType::End => panic!("{}", error_not_found::<Var::Key>()),
        VariableListType::Removed
            if type_eq::<Var::Key, <List::Variable as ConstVariable>::Key>() =>
        {
            panic!("{}", error_not_found::<Var::Key>())
        }
        VariableListType::Has if type_eq::<Var::Key, <List::Variable as ConstVariable>::Key>() => {
            assert!(
                type_eq::<Var::Value, <List::Variable as ConstVariable>::Value>(),
                "{}",
                error_unexpected_type::<Var::Value, <List::Variable as ConstVariable>::Value>()
            );

            unsafe {
                let value = List::VALUE;
                let ret = core::mem::transmute_copy(&value);
                core::mem::forget(value);
                ret
            }
        }
        _ => find_variable::<List::Next, Var>(),
    }
}

#[track_caller]
pub const fn is_variable_in<List, Var>() -> bool
where
    List: VariableList,
    Var: ConstVariable,
{
    match List::TYPE {
        VariableListType::End => false,
        VariableListType::Removed
            if type_eq::<Var::Key, <List::Variable as ConstVariable>::Key>() =>
        {
            false
        }
        VariableListType::Has if type_eq::<Var::Key, <List::Variable as ConstVariable>::Key>() => {
            assert!(
                type_eq::<Var::Value, <List::Variable as ConstVariable>::Value>(),
                "{}",
                error_unexpected_type::<Var::Value, <List::Variable as ConstVariable>::Value>()
            );
            true
        }
        _ => is_variable_in::<List::Next, Var>(),
    }
}
