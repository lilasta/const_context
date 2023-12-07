use core::marker::PhantomData;

use crate::action::{Action, ConstContext};
use crate::effect::{Effect, EffectListHas};

#[derive(Clone, Copy)]
pub struct SetEffectAction<Eff, EffConcrete>(PhantomData<(Eff, EffConcrete)>);

impl<Eff, EffConcrete> SetEffectAction<Eff, EffConcrete> {
    #[inline(always)]
    pub fn new(_: EffConcrete) -> Self {
        Self(PhantomData)
    }
}

impl<Eff, EffConcrete> Action for SetEffectAction<Eff, EffConcrete>
where
    Eff: Effect,
    EffConcrete: 'static + Fn<Eff::Args, Output = Eff::Output>,
{
    type Output = ();
    type Context<Ctx: ConstContext> = (
        Ctx::Strictness,
        EffectListHas<Eff, EffConcrete, Ctx::Effects>,
        Ctx::Variables,
    );

    #[inline(always)]
    fn run_with<Ctx: ConstContext>(self) -> Self::Output {}
}
