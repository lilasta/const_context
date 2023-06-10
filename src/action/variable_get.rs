use core::marker::PhantomData;

use crate::action::{Action, ActionContext};
use crate::variable::{find_variable, ConstVariable};

pub struct GetAction<Variable>(PhantomData<Variable>);

impl<Variable> GetAction<Variable> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Variable> Action for GetAction<Variable>
where
    Variable: ConstVariable,
{
    type Output = Variable::Value;
    type Context<Ctx: ActionContext> = Ctx;

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {
        const { find_variable::<Ctx::Variables, Variable::Key, Variable::Value>() }
    }
}
