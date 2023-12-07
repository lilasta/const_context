use crate::effect::{EffectList, EffectListEnd};
use crate::value::bool::{ConstBool, ConstTrue};
use crate::variable::list::{VariableList, VariableListEmpty};

pub type InitialActionContext = (ConstTrue, EffectListEnd, VariableListEmpty);

pub trait ConstContext {
    type Strictness: ConstBool;
    type Effects: EffectList;
    type Variables: VariableList;
}

impl<S, E, V> ConstContext for (S, E, V)
where
    S: ConstBool,
    E: EffectList,
    V: VariableList,
{
    type Strictness = S;
    type Effects = E;
    type Variables = V;
}
