use crate::action::{Action, ConstContext};

#[derive(Clone, Copy)]
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
    type Context<Ctx: ConstContext> = NextAction::Context<PreviousAction::Context<Ctx>>;

    #[inline(always)]
    fn run_with<Ctx: ConstContext>(self) -> Self::Output {
        let Self(action, constructor) = self;
        let output = action.run_with::<Ctx>();
        constructor(output).run_with::<PreviousAction::Context<Ctx>>()
    }
}
