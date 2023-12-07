use crate::action::{Action, ConstContext};

#[derive(Clone, Copy)]
pub struct PureAction<Value>(Value);

impl<Value> PureAction<Value> {
    #[inline(always)]
    pub const fn new(value: Value) -> Self {
        Self(value)
    }
}

impl<Value> Action for PureAction<Value> {
    type Output = Value;
    type Context<Ctx: ConstContext> = Ctx;

    #[inline(always)]
    fn run_with<Ctx: ConstContext>(self) -> Self::Output {
        self.0
    }
}
