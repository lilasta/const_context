use crate::action::{Action, ConstContext};

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
    type Context<Ctx: ConstContext> = NextAction::Context<PreviousAction::Context<Ctx>>;

    #[inline(always)]
    fn run_with<Ctx: ConstContext>(self) -> Self::Output {
        let Self(rt_ctx, action, constructor) = self;
        let output = action.run_with::<Ctx>();
        constructor(output, rt_ctx).run_with::<PreviousAction::Context<Ctx>>()
    }
}
