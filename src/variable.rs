use core::marker::PhantomData;

use crate::utils::{str_concat, type_eq};
use crate::value::ConstValue;

pub trait ConstVariable {
    type Key: 'static;
    type Value: 'static;
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

pub struct VariableListHas<Key, Value, const VALUE: ConstValue, Next>(
    PhantomData<(Key, Value, Next)>,
);

pub struct VariableListRemoved<Key, Next>(PhantomData<(Key, Next)>);

pub enum VariableListValue<T> {
    End,
    Has(T),
    Removed,
}

pub trait VariableList {
    type Next: VariableList;
    type Key: 'static;
    type Value: 'static;
    const VALUE: VariableListValue<ConstValue>;
}

impl VariableList for VariableListEnd {
    type Next = VariableListEnd;
    type Key = ();
    type Value = ();
    const VALUE: VariableListValue<ConstValue> = VariableListValue::End;
}

impl<Key: 'static, Value: 'static, const VAL: ConstValue, Next: VariableList> VariableList
    for VariableListHas<Key, Value, VAL, Next>
{
    type Next = Next;
    type Key = Key;
    type Value = Value;
    const VALUE: VariableListValue<ConstValue> = VariableListValue::Has(VAL);
}

impl<Key: 'static, Next: VariableList> VariableList for VariableListRemoved<Key, Next> {
    type Next = Next;
    type Key = Key;
    type Value = ();
    const VALUE: VariableListValue<ConstValue> = VariableListValue::Removed;
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
pub const fn find_variable<List, Key, Value>() -> Value
where
    List: VariableList,
    Key: 'static,
    Value: 'static,
{
    match List::VALUE {
        VariableListValue::End => panic!("{}", error_not_found::<Key>()),
        VariableListValue::Removed if type_eq::<Key, List::Key>() => {
            panic!("{}", error_not_found::<Key>())
        }
        VariableListValue::Has(value) if type_eq::<Key, List::Key>() => {
            assert!(
                type_eq::<Value, List::Value>(),
                "{}",
                error_unexpected_type::<Value, List::Value>()
            );
            value.with_type()
        }
        _ => find_variable::<List::Next, Key, Value>(),
    }
}

#[track_caller]
pub const fn is_variable_in<List, Key, Value>() -> bool
where
    List: VariableList,
    Key: 'static,
    Value: 'static,
{
    match List::VALUE {
        VariableListValue::End => false,
        VariableListValue::Removed if type_eq::<Key, List::Key>() => false,
        VariableListValue::Has(_) if type_eq::<Key, List::Key>() => {
            assert!(
                type_eq::<Value, List::Value>(),
                "{}",
                error_unexpected_type::<Value, List::Value>()
            );
            true
        }
        _ => is_variable_in::<List::Next, Key, Value>(),
    }
}
