use core::marker::PhantomData;

use crate::action::{Action, ActionContext};
use crate::value::ConstValue;
use crate::variable::{ConstVariable, VariableListHas};

pub struct SetAction<Var, Value>(PhantomData<(Var, Value)>);

impl<Var, Value> SetAction<Var, Value> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Var, Value> Action for SetAction<Var, Value>
where
    Var: ConstVariable,
    Value: ConstValue<Type = Var::Value>,
{
    type Output = ();
    type Context<Ctx: ActionContext> = (Ctx::Effects, VariableListHas<Var, Value, Ctx::Variables>);

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {}
}
