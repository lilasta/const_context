use core::marker::PhantomData;

use crate::action::{Action, ActionContext};

pub struct PanicAction<Msg>(PhantomData<Msg>);

impl<Msg> PanicAction<Msg> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Msg> Action for PanicAction<Msg>
where
    Msg: PanicMessage,
{
    type Output = !;
    type Context<Ctx: ActionContext> = Ctx;

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {
        const { panic!("{}", Msg::MSG) }
    }
}

pub trait PanicMessage {
    const MSG: &'static str;
}
