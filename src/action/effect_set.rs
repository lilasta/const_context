use core::marker::PhantomData;

use crate::action::Action;
use crate::effect::{Effect, EffectList, EffectListHas};
use crate::variable::VariableList;

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
    type Effects<Effects: EffectList> =
        EffectListHas<Function::Name, Function::Args, FunctionConcrete, Effects>;
    type Vars<Vars: VariableList> = Vars;

    #[inline(always)]
    fn eval<Effects: EffectList, Vars: VariableList>(self) -> Self::Output {}
}
