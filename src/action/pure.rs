use crate::action::{Action, ActionContext};

pub struct PureAction<Value>(Value);

impl<Value> PureAction<Value> {
    #[inline(always)]
    pub const fn new(value: Value) -> Self {
        Self(value)
    }
}

impl<Value> Action for PureAction<Value> {
    type Output = Value;
    type Context<Ctx: ActionContext> = Ctx;

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {
        self.0
    }
}
