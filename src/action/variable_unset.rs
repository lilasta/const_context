use core::marker::PhantomData;

use crate::action::{Action, ConstContext};
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
    type Context<Ctx: ConstContext> = (
        Ctx::Strictness,
        Ctx::Effects,
        VariableListRemove<Var, Ctx::Variables>,
    );

    #[inline(always)]
    fn run_with<Ctx: ConstContext>(self) -> Self::Output {}
}
