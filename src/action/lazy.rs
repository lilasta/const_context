use crate::action::Action;
use crate::effect::EffectList;
use crate::variable::VariableList;

pub struct LazyAction<Closure>(Closure);

impl<Closure> LazyAction<Closure> {
    #[inline(always)]
    pub const fn new(closure: Closure) -> Self {
        Self(closure)
    }
}

impl<Closure, A> Action for LazyAction<Closure>
where
    Closure: FnOnce() -> A,
    A: Action,
{
    type Output = A::Output;
    type Effects<Effects: EffectList> = A::Effects<Effects>;
    type Vars<Vars: VariableList> = A::Vars<Vars>;

    #[inline(always)]
    fn eval<Effects: EffectList, Vars: VariableList>(self) -> Self::Output {
        let Self(closure) = self;
        closure().eval::<Effects, Vars>()
    }
}
