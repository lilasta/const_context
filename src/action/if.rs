use core::marker::PhantomData;

use crate::action::{Action, ActionContext};
use crate::condition::Condition;
use crate::value::bool::{ConstAnd, ConstNot};
use crate::value::ConstValue;
use crate::variable::list::VariableListIf;

pub struct IfAction<Cond, A, B>(A, B, PhantomData<Cond>);

impl<Cond, A, B> IfAction<Cond, A, B>
where
    Cond: Condition,
    A: Action,
    B: Action<Output = A::Output>,
{
    #[inline(always)]
    pub const fn new(a: A, b: B) -> Self {
        Self(a, b, PhantomData)
    }
}

impl<Cond, A, B> Action for IfAction<Cond, A, B>
where
    Cond: Condition,
    A: Action,
    B: Action<Output = A::Output>,
{
    type Output = A::Output;
    type Context<Ctx: ActionContext> = (
        Ctx::Strictness,
        Ctx::Effects, // TODO
        VariableListIf<
            Cond::Bool<Ctx>,
            <A::Context<Ctx> as ActionContext>::Variables,
            <B::Context<Ctx> as ActionContext>::Variables,
        >,
    );

    #[inline(always)]
    fn run_with<Ctx: ActionContext>(self) -> Self::Output {
        let Self(a, b, ..) = self;
        if const { <Cond::Bool<Ctx> as ConstValue>::VALUE } {
            a.run_with::<(
                ConstAnd<Ctx::Strictness, Cond::Bool<Ctx>>,
                Ctx::Effects,
                Ctx::Variables,
            )>()
        } else {
            b.run_with::<(
                ConstAnd<Ctx::Strictness, ConstNot<Cond::Bool<Ctx>>>,
                Ctx::Effects,
                Ctx::Variables,
            )>()
        }
    }
}
