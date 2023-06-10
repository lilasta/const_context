use crate::action::Action;
use crate::effect::EffectList;
use crate::variable::VariableList;

pub struct PureAction<Value>(Value);

impl<Value> PureAction<Value> {
    #[inline(always)]
    pub const fn new(value: Value) -> Self {
        Self(value)
    }
}

impl<Value> Action for PureAction<Value> {
    type Output = Value;
    type Effects<Effects: EffectList> = Effects;
    type Vars<Vars: VariableList> = Vars;

    #[inline(always)]
    fn eval<Effects: EffectList, Vars: VariableList>(self) -> Self::Output {
        self.0
    }
}
