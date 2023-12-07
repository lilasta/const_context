use crate::action::{Action, ConstContext};

#[derive(Clone, Copy)]
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
    type Context<Ctx: ConstContext> = A::Context<Ctx>;

    #[inline(always)]
    fn run_with<Ctx: ConstContext>(self) -> Self::Output {
        let Self(closure) = self;
        closure().run_with::<Ctx>()
    }
}
