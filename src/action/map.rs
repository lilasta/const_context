use crate::action::{Action, ConstContext};

#[derive(Clone, Copy)]
pub struct MapAction<A, F>(A, F);

impl<A, F> MapAction<A, F> {
    #[inline(always)]
    pub const fn new(action: A, f: F) -> Self {
        Self(action, f)
    }
}

impl<A, F, R> Action for MapAction<A, F>
where
    A: Action,
    F: FnOnce(A::Output) -> R,
{
    type Output = R;
    type Context<Ctx: ConstContext> = A::Context<Ctx>;

    #[inline(always)]
    fn run_with<Ctx: ConstContext>(self) -> Self::Output {
        let Self(action, f) = self;
        let x = action.run_with::<Ctx>();
        f(x)
    }
}
