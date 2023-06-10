pub mod bind;
pub mod effect_get;
pub mod effect_set;
pub mod lazy;
pub mod panic;
pub mod pure;
pub mod variable_get;
pub mod variable_set;
pub mod variable_unset;

use crate::effect::{EffectList, EffectListEnd};
use crate::variable::{VariableList, VariableListEnd};

pub trait Action: Sized {
    type Output;

    type Context<Ctx: ActionContext>: ActionContext;
    fn eval<Ctx: ActionContext>(self) -> Self::Output;

    #[inline(always)]
    fn start_eval(self) -> Self::Output {
        self.eval::<(EffectListEnd, VariableListEnd)>()
    }
}

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
