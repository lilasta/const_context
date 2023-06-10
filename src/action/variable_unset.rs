use core::marker::PhantomData;

use crate::action::{Action, ActionContext};
use crate::variable::{ConstVariable, VariableListRemoved};

pub struct UnsetAction<Variable>(PhantomData<Variable>);

impl<Variable> UnsetAction<Variable> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Variable> Action for UnsetAction<Variable>
where
    Variable: ConstVariable,
{
    type Output = ();
    type Context<Ctx: ActionContext> = (
        Ctx::Effects,
        VariableListRemoved<Variable::Key, Ctx::Variables>,
    );

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {}
}
