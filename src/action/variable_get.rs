use core::marker::PhantomData;

use crate::action::{Action, ActionContext};
use crate::variable::list::find_variable;
use crate::variable::Variable;

#[derive(Clone, Copy)]
pub struct GetAction<Var>(PhantomData<Var>);

impl<Variable> GetAction<Variable> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Var> Action for GetAction<Var>
where
    Var: Variable,
{
    type Output = Var::Type;
    type Context<Ctx: ActionContext> = Ctx;

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {
        const { find_variable::<Ctx::Variables, Var>() }
    }
}
