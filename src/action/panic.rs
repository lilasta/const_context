use core::marker::PhantomData;

use crate::action::{Action, ActionContext};

pub struct PanicAction<const MSG: &'static str, T>(PhantomData<T>);

impl<const MSG: &'static str, T> PanicAction<MSG, T> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<const MSG: &'static str, T> Action for PanicAction<MSG, T> {
    type Output = T;
    type Context<Ctx: ActionContext> = Ctx;

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {
        const { panic!("{}", MSG) }
    }
}
