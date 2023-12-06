use core::marker::PhantomData;

use crate::action::{Action, ActionContext};
use crate::value::bool::strict_if;
use crate::value::ConstValue;
use crate::variable::list::VariableListCons;
use crate::variable::Variable;

#[derive(Clone, Copy)]
pub struct SetAction<Var, Value>(PhantomData<(Var, Value)>);

impl<Var, Value> SetAction<Var, Value> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Var, Value> Action for SetAction<Var, Value>
where
    Var: Variable,
    Value: ConstValue<Type = Var::Type>,
{
    type Output = ();
    type Context<Ctx: ActionContext> = (
        Ctx::Strictness,
        Ctx::Effects,
        VariableListCons<Var, Value, Ctx::Variables>,
    );

    #[inline(always)]
    fn run_with<Ctx: ActionContext>(self) -> Self::Output {
        const { strict_if::<Ctx::Strictness, Value>() }
    }
}
