use core::marker::PhantomData;

use crate::action::{Action, ActionContext};
use crate::effect::{Effect, EffectListHas};

pub struct SetEffectAction<Function, FunctionConcrete>(PhantomData<(Function, FunctionConcrete)>);

impl<Function, FunctionConcrete> SetEffectAction<Function, FunctionConcrete> {
    #[inline(always)]
    pub fn new(_: FunctionConcrete) -> Self {
        Self(PhantomData)
    }
}

impl<Function, FunctionConcrete> Action for SetEffectAction<Function, FunctionConcrete>
where
    Function: Effect,
    FunctionConcrete: 'static + Fn<Function::Args, Output = Function::Ret>,
{
    type Output = ();
    type Context<Ctx: ActionContext> = (
        EffectListHas<Function::Name, Function::Args, FunctionConcrete, Ctx::Effects>,
        Ctx::Variables,
    );

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {}
}
