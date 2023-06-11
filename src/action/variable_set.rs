use core::marker::PhantomData;

use crate::action::{Action, ActionContext};
use crate::value::ConstValue;
use crate::variable::{ConstVariable, VariableListHas};

pub struct SetAction<Variable, const VALUE: ConstValue>(PhantomData<Variable>);

impl<Variable, const VALUE: ConstValue> SetAction<Variable, VALUE> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Variable, const VALUE: ConstValue> Action for SetAction<Variable, VALUE>
where
    Variable: ConstVariable,
{
    type Output = ();
    type Context<Ctx: ActionContext> = (
        Ctx::Effects,
        VariableListHas<Variable, VALUE, Ctx::Variables>,
    );

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {}
}
