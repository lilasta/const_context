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

pub enum VariableListValue<T> {
    End,
    Has(T),
    Removed,
}

pub trait VariableList {
    type Next: VariableList;
    type Variable: ConstVariable;
    const VALUE: VariableListValue<<Self::Variable as ConstVariable>::Value>;
}

impl VariableList for VariableListEnd {
    type Next = VariableListEnd;
    type Variable = ();
    const VALUE: VariableListValue<<Self::Variable as ConstVariable>::Value> =
        VariableListValue::End;
}

impl<Var, Value, Next> VariableList for VariableListHas<Var, Value, Next>
where
    Var: ConstVariable,
    Value: ConstValue<Type = Var::Value>,
    Next: VariableList,
{
    type Next = Next;
    type Variable = Var;
    const VALUE: VariableListValue<Var::Value> = VariableListValue::Has(Value::VALUE);
}

impl<Var, Next> VariableList for VariableListRemoved<Var, Next>
where
    Var: ConstVariable,
    Next: VariableList,
{
    type Next = Next;
    type Variable = Var;
    const VALUE: VariableListValue<Var::Value> = VariableListValue::Removed;
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
    match List::VALUE {
        VariableListValue::End => panic!("{}", error_not_found::<Var::Key>()),
        VariableListValue::Removed
            if type_eq::<Var::Key, <List::Variable as ConstVariable>::Key>() =>
        {
            panic!("{}", error_not_found::<Var::Key>())
        }
        VariableListValue::Has(value)
            if type_eq::<Var::Key, <List::Variable as ConstVariable>::Key>() =>
        {
            assert!(
                type_eq::<Var::Value, <List::Variable as ConstVariable>::Value>(),
                "{}",
                error_unexpected_type::<Var::Value, <List::Variable as ConstVariable>::Value>()
            );

            unsafe {
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
    match List::VALUE {
        VariableListValue::End => false,
        VariableListValue::Removed
            if type_eq::<Var::Key, <List::Variable as ConstVariable>::Key>() =>
        {
            false
        }
        VariableListValue::Has(_)
            if type_eq::<Var::Key, <List::Variable as ConstVariable>::Key>() =>
        {
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
