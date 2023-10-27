use core::marker::{Destruct, PhantomData};

use crate::utils::{is_same_type, str_concat_ct};
use crate::value::ConstValue;

pub trait Variable {
    type Key: 'static;
    type Value: 'static + ~const Destruct;
}

impl<K, V> Variable for (K, V)
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
    type Var: Variable;
    const TYPE: VariableListType;
    const VALUE: <Self::Var as Variable>::Value;
}

impl VariableList for VariableListEnd {
    type Next = VariableListEnd;
    type Var = (!, !);
    const TYPE: VariableListType = VariableListType::End;
    const VALUE: <Self::Var as Variable>::Value = unreachable!();
}

impl<Var, Value, Next> VariableList for VariableListHas<Var, Value, Next>
where
    Var: Variable,
    Value: ConstValue<Type = Var::Value>,
    Next: VariableList,
{
    type Next = Next;
    type Var = Var;
    const TYPE: VariableListType = VariableListType::Has;
    const VALUE: Var::Value = Value::VALUE;
}

impl<Var, Next> VariableList for VariableListRemoved<Var, Next>
where
    Var: Variable,
    Next: VariableList,
{
    type Next = Next;
    type Var = Var;
    const TYPE: VariableListType = VariableListType::Removed;
    const VALUE: Var::Value = unreachable!();
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

#[track_caller]
pub const fn find_variable<List, Var>() -> Var::Value
where
    List: VariableList,
    Var: Variable,
{
    match List::TYPE {
        VariableListType::End => panic!("{}", error_not_found::<Var::Key>()),
        VariableListType::Removed if is_same_type::<Var::Key, <List::Var as Variable>::Key>() => {
            panic!("{}", error_not_found::<Var::Key>())
        }
        VariableListType::Has if is_same_type::<Var::Key, <List::Var as Variable>::Key>() => {
            assert!(
                is_same_type::<Var::Value, <List::Var as Variable>::Value>(),
                "{}",
                error_unexpected_type::<Var::Value, <List::Var as Variable>::Value>()
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
    Var: Variable,
{
    match List::TYPE {
        VariableListType::End => false,
        VariableListType::Removed if is_same_type::<Var::Key, <List::Var as Variable>::Key>() => {
            false
        }
        VariableListType::Has if is_same_type::<Var::Key, <List::Var as Variable>::Key>() => {
            assert!(
                is_same_type::<Var::Value, <List::Var as Variable>::Value>(),
                "{}",
                error_unexpected_type::<Var::Value, <List::Var as Variable>::Value>()
            );
            true
        }
        _ => is_variable_in::<List::Next, Var>(),
    }
}
