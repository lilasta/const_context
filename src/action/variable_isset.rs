use core::marker::PhantomData;

use crate::action::{Action, ConstContext};
use crate::variable::list::is_variable_in;
use crate::variable::Variable;

#[derive(Clone, Copy)]
pub struct IsSetAction<Var>(PhantomData<Var>);

impl<Var> IsSetAction<Var> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Var> Action for IsSetAction<Var>
where
    Var: Variable,
{
    type Output = bool;
    type Context<Ctx: ConstContext> = Ctx;

    #[inline(always)]
    fn run_with<Ctx: ConstContext>(self) -> Self::Output {
        const { is_variable_in::<Ctx::Variables, Var>() }
    }
}
