use core::marker::PhantomData;

use crate::action::ActionContext;
use crate::value::bool::ConstBool;
use crate::value::ConstValue;
use crate::variable::list::{is_variable_in, VariableList};
use crate::variable::Variable;

pub trait Condition {
    type Bool<Ctx: ActionContext>: ConstBool;
}

pub struct IsVariableIn<Var: Variable>(PhantomData<Var>);

impl<Var: Variable> Condition for IsVariableIn<Var> {
    type Bool<Ctx: ActionContext> = IsVariableIn_<Var, Ctx::Variables>;
}

pub struct IsVariableIn_<Var: Variable, Vars: VariableList>(PhantomData<(Var, Vars)>);

impl<Var: Variable, Vars: VariableList> ConstValue for IsVariableIn_<Var, Vars> {
    type Type = bool;
    const VALUE: Self::Type = is_variable_in::<Vars, Var>();
}
