use core::marker::PhantomData;

use crate::action::{Action, ActionContext};
use crate::variable::list::VariableListRemove;
use crate::variable::Variable;

#[derive(Clone, Copy)]
pub struct UnsetAction<Var>(PhantomData<Var>);

impl<Var> UnsetAction<Var> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Var> Action for UnsetAction<Var>
where
    Var: Variable,
{
    type Output = ();
    type Context<Ctx: ActionContext> = (Ctx::Effects, VariableListRemove<Var, Ctx::Variables>);

    #[inline(always)]
    fn run_with<Ctx: ActionContext>(self) -> Self::Output {}
}
