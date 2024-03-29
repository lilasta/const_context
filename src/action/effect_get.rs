use core::marker::PhantomData;

use crate::action::{Action, ConstContext};
use crate::effect::{Effect, RuntimeEffect};

#[derive(Clone, Copy)]
pub struct GetEffectAction<Eff>(PhantomData<Eff>);

impl<Eff> GetEffectAction<Eff> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Eff> Action for GetEffectAction<Eff>
where
    Eff: Effect,
{
    type Output = RuntimeEffect<Eff>;
    type Context<Ctx: ConstContext> = Ctx;

    #[inline(always)]
    fn run_with<Ctx: ConstContext>(self) -> Self::Output {
        crate::effect::get::<Ctx::Effects, Eff>()
    }
}
