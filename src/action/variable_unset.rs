use core::marker::PhantomData;

use crate::action::Action;
use crate::effect::EffectList;
use crate::variable::{ConstVariable, VariableList, VariableListRemoved};

pub struct UnsetAction<Variable>(PhantomData<Variable>);

impl<Variable> UnsetAction<Variable> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Variable> Action for UnsetAction<Variable>
where
    Variable: ConstVariable,
{
    type Output = ();
    type Effects<Effects: EffectList> = Effects;
    type Vars<Vars: VariableList> = VariableListRemoved<Variable::Key, Vars>;

    #[inline(always)]
    fn eval<Effects: EffectList, Vars: VariableList>(self) -> Self::Output {}
}
