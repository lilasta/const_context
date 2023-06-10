use core::marker::PhantomData;

use crate::action::{Action, ActionContext};
use crate::effect::Effect;

pub struct GetEffectAction<Function>(PhantomData<Function>);

impl<Function> GetEffectAction<Function> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Function> Action for GetEffectAction<Function>
where
    Function: Effect,
{
    type Output = CallWrapper<Function::Args, Function::Ret>;
    type Context<Ctx: ActionContext> = Ctx;

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {
        CallWrapper(
            crate::effect::call::<Ctx::Effects, Function::Name, Function::Args, Function::Ret>,
        )
    }
}

pub struct CallWrapper<Args, Ret>(fn(Args) -> Ret);

impl<Args: core::marker::Tuple, Ret> FnOnce<Args> for CallWrapper<Args, Ret> {
    type Output = Ret;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        self.0.call_once((args,))
    }
}

impl<Args: core::marker::Tuple, Ret> FnMut<Args> for CallWrapper<Args, Ret> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        self.0.call_mut((args,))
    }
}

impl<Args: core::marker::Tuple, Ret> Fn<Args> for CallWrapper<Args, Ret> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        self.0.call((args,))
    }
}