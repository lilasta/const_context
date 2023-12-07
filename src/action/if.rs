use core::marker::PhantomData;

use crate::action::{Action, ActionContext};
use crate::condition::Condition;
use crate::value::bool::{ConstAnd, ConstNot};
use crate::value::ConstValue;
use crate::variable::list::VariableListIf;

pub struct IfAction<Cond, A, B, RuntimeContext>(A, B, RuntimeContext, PhantomData<Cond>);

impl<Cond, A, B, RuntimeContext> IfAction<Cond, A, B, RuntimeContext> {
    #[inline(always)]
    pub const fn new<ActionA, ActionB>(rt_ctx: RuntimeContext, a: A, b: B) -> Self
    where
        A: FnOnce(RuntimeContext) -> ActionA,
        B: FnOnce(RuntimeContext) -> ActionB,
    {
        Self(a, b, rt_ctx, PhantomData)
    }
}

impl<Cond, A, B, ActionA, ActionB, RuntimeContext> Action for IfAction<Cond, A, B, RuntimeContext>
where
    Cond: Condition,
    A: FnOnce(RuntimeContext) -> ActionA,
    B: FnOnce(RuntimeContext) -> ActionB,
    ActionA: Action,
    ActionB: Action<Output = ActionA::Output>,
{
    type Output = ActionA::Output;
    type Context<Ctx: ActionContext> = (
        Ctx::Strictness,
        Ctx::Effects, // TODO
        VariableListIf<
            Cond::Bool<Ctx>,
            <ActionA::Context<Ctx> as ActionContext>::Variables,
            <ActionB::Context<Ctx> as ActionContext>::Variables,
        >,
    );

    #[inline(always)]
    fn run_with<Ctx: ActionContext>(self) -> Self::Output {
        let Self(a, b, rt_ctx, ..) = self;
        if const { <Cond::Bool<Ctx> as ConstValue>::VALUE } {
            a(rt_ctx).run_with::<(
                ConstAnd<Ctx::Strictness, Cond::Bool<Ctx>>,
                Ctx::Effects,
                Ctx::Variables,
            )>()
        } else {
            b(rt_ctx).run_with::<(
                ConstAnd<Ctx::Strictness, ConstNot<Cond::Bool<Ctx>>>,
                Ctx::Effects,
                Ctx::Variables,
            )>()
        }
    }
}
