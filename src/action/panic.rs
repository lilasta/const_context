use core::marker::PhantomData;

use crate::action::{Action, ActionContext};

#[derive(Clone, Copy)]
pub struct PanicAction<Msg, T = !>(PhantomData<(Msg, T)>);

impl<Msg, T> PanicAction<Msg, T> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Msg, T> Action for PanicAction<Msg, T>
where
    Msg: PanicMessage,
{
    type Output = T;
    type Context<Ctx: ActionContext> = Ctx;

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {
        const {
            panic!("{}", Msg::MSG);
        }
    }
}

pub trait PanicMessage {
    const MSG: &'static str;
}
