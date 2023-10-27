use crate::action::{Action, ActionContext};

#[derive(Clone, Copy)]
pub struct BindAction<RuntimeContext, PreviousAction, ActionConstructor>(
    RuntimeContext,
    PreviousAction,
    ActionConstructor,
);

impl<RuntimeContext, PreviousAction, ActionConstructor>
    BindAction<RuntimeContext, PreviousAction, ActionConstructor>
{
    #[inline(always)]
    pub const fn new<Ret>(
        prev: PreviousAction,
        rt_ctx: RuntimeContext,
        constructor: ActionConstructor,
    ) -> Self
    where
        PreviousAction: Action,
        ActionConstructor: FnOnce(PreviousAction::Output, RuntimeContext) -> Ret,
    {
        Self(rt_ctx, prev, constructor)
    }
}

impl<RuntimeContext, PreviousAction, ActionConstructor, NextAction> Action
    for BindAction<RuntimeContext, PreviousAction, ActionConstructor>
where
    PreviousAction: Action,
    ActionConstructor: FnOnce(PreviousAction::Output, RuntimeContext) -> NextAction,
    NextAction: Action,
{
    type Output = NextAction::Output;
    type Context<Ctx: ActionContext> = NextAction::Context<PreviousAction::Context<Ctx>>;

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {
        let Self(rt_ctx, action, constructor) = self;
        let output = action.eval::<Ctx>();
        constructor(output, rt_ctx).eval::<PreviousAction::Context<Ctx>>()
    }
}
