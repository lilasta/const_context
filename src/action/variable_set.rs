use core::marker::PhantomData;

use crate::action::Action;
use crate::effect::EffectList;
use crate::value::ConstValue;
use crate::variable::{ConstVariable, VariableList, VariableListHas};

pub struct SetAction<Variable, const VALUE: ConstValue>(PhantomData<Variable>);

impl<Variable, const VALUE: ConstValue> SetAction<Variable, VALUE> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Variable, const VALUE: ConstValue> Action for SetAction<Variable, VALUE>
where
    Variable: ConstVariable,
{
    type Output = ();
    type Effects<Effects: EffectList> = Effects;
    type Vars<Vars: VariableList> = VariableListHas<Variable::Key, Variable::Value, VALUE, Vars>;

    #[inline(always)]
    fn eval<Effects: EffectList, Vars: VariableList>(self) -> Self::Output {}
}
