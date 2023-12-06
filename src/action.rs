pub mod bind;
pub mod effect_get;
pub mod effect_set;
pub mod r#if;
pub mod lazy;
pub mod pure;
pub mod variable_get;
pub mod variable_set;
pub mod variable_unset;

use crate::effect::{EffectList, EffectListEnd};
use crate::value::bool::{ConstBool, ConstTrue};
use crate::variable::list::{VariableList, VariableListEmpty};

pub type InitialActionContext = (ConstTrue, EffectListEnd, VariableListEmpty);

pub trait ActionContext {
    type Strictness: ConstBool;
    type Effects: EffectList;
    type Variables: VariableList;
}

impl<S, E, V> ActionContext for (S, E, V)
where
    S: ConstBool,
    E: EffectList,
    V: VariableList,
{
    type Strictness = S;
    type Effects = E;
    type Variables = V;
}

pub trait Action: Sized {
    type Output;
    type Context<Ctx: ActionContext>: ActionContext;

    #[inline(always)]
    fn run(self) -> Self::Output {
        self.run_with::<InitialActionContext>()
    }

    fn run_with<Ctx: ActionContext>(self) -> Self::Output;
}
