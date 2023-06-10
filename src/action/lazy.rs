use crate::action::{Action, ActionContext};
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
    type Context<Ctx: ActionContext> = A::Context<Ctx>;

    #[inline(always)]
    fn eval<Ctx: ActionContext>(self) -> Self::Output {
        let Self(closure) = self;
        closure().eval::<Ctx>()
    }
}
