use core::marker::PhantomData;

use crate::action::Action;
use crate::effect::EffectList;
use crate::variable::{find_variable, ConstVariable, VariableList};

pub struct GetAction<Variable>(PhantomData<Variable>);

impl<Variable> GetAction<Variable> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Variable> Action for GetAction<Variable>
where
    Variable: ConstVariable,
{
    type Output = Variable::Value;
    type Effects<Effects: EffectList> = Effects;
    type Vars<Vars: VariableList> = Vars;

    #[inline(always)]
    fn eval<Effects: EffectList, Vars: VariableList>(self) -> Self::Output {
        const { find_variable::<Vars, Variable::Key, Variable::Value>() }
    }
}
