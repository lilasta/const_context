use crate::action::Action;
use crate::effect::EffectList;
use crate::variable::VariableList;

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
    type Effects<Effects: EffectList> = NextAction::Effects<PreviousAction::Effects<Effects>>;
    type Vars<Vars: VariableList> = NextAction::Vars<PreviousAction::Vars<Vars>>;

    #[inline(always)]
    fn eval<Effects: EffectList, Vars: VariableList>(self) -> Self::Output {
        let Self(action, constructor) = self;
        let output = action.eval::<Effects, Vars>();
        constructor(output).eval::<PreviousAction::Effects<Effects>, PreviousAction::Vars<Vars>>()
    }
}
