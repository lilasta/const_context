use crate::action::{Action, ActionContext};

pub struct BindAction<PreviousAction, ActionConstructor>(PreviousAction, ActionConstructor);

impl<PreviousAction, ActionConstructor> BindAction<PreviousAction, ActionConstructor> {
    #[inline(always)]
    pub const fn new<Ret>(prev: PreviousAction, constructor: ActionConstructor) -> Self
    where
        PreviousAction: Action,
        ActionConstructor: FnOnce(PreviousAction::Output) -> Ret,
    {
        Self(prev, constructor)
    }
}

impl<PreviousAction, ActionConstructor, NextAction> Action
    for BindAction<PreviousAction, ActionConstructor>
where
    PreviousAction: Action,
    ActionConstructor: FnOnce(PreviousAction::Output) -> NextAction,
    NextAction: Action,
{
    type Output = NextAction::Output;
    type Context<Ctx: ActionContext> = NextAction::Context<PreviousAction::Context<Ctx>>;

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {
        let Self(action, constructor) = self;
        let output = action.eval::<Ctx>();
        constructor(output).eval::<PreviousAction::Context<Ctx>>()
    }
}
