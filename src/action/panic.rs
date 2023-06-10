use core::marker::PhantomData;

use crate::action::Action;
use crate::effect::EffectList;
use crate::variable::VariableList;

pub struct PanicAction<const MSG: &'static str, T>(PhantomData<T>);

impl<const MSG: &'static str, T> PanicAction<MSG, T> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<const MSG: &'static str, T> Action for PanicAction<MSG, T> {
    type Output = T;
    type Effects<Effects: EffectList> = Effects;
    type Vars<Vars: VariableList> = Vars;

    #[inline(always)]
    fn eval<Effects: EffectList, Vars: VariableList>(self) -> Self::Output {
        const { panic!("{}", MSG) }
    }
}
