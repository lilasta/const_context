pub mod bind;
pub mod effect_get;
pub mod effect_set;
pub mod lazy;
pub mod pure;
pub mod variable_get;
pub mod variable_set;
pub mod variable_unset;

use crate::effect::{EffectList, EffectListEnd};
use crate::variable::list::{VariableList, VariableListEmpty};

pub type InitialActionContext = (EffectListEnd, VariableListEmpty);

pub trait ActionContext {
    type Effects: EffectList;
    type Variables: VariableList;
}

impl<E, V> ActionContext for (E, V)
where
    E: EffectList,
    V: VariableList,
{
    type Effects = E;
    type Variables = V;
}

pub trait Action: Sized {
    type Output;
    type Context<Ctx: ActionContext>: ActionContext;

    #[inline(always)]
    fn start_eval(self) -> Self::Output {
        self.eval::<InitialActionContext>()
    }

    fn eval<Ctx: ActionContext>(self) -> Self::Output;
}
