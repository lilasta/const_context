use core::marker::PhantomData;

use crate::action::{Action, ActionContext};
use crate::value::bool::{ConstBool, ConstNot};
use crate::value::ConstValue;
use crate::variable::list::{is_variable_in, VariableList, VariableListIf};
use crate::variable::Variable;

pub trait Condition {
    type Bool<Ctx: ActionContext>: ConstBool;
}

pub struct IsVariableIn<Var: Variable>(PhantomData<Var>);

impl<Var: Variable> Condition for IsVariableIn<Var> {
    type Bool<Ctx: ActionContext> = ValueOfIsVariableIn<Var, Ctx::Variables>;
}

pub struct ValueOfIsVariableIn<Var: Variable, Vars: VariableList>(PhantomData<(Var, Vars)>);

impl<Var: Variable, Vars: VariableList> ConstValue for ValueOfIsVariableIn<Var, Vars> {
    type Type = bool;
    const VALUE: Self::Type = is_variable_in::<Vars, Var>();
}

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
            a.run_with::<(Cond::Bool<Ctx>, Ctx::Effects, Ctx::Variables)>()
        } else {
            b.run_with::<(ConstNot<Cond::Bool<Ctx>>, Ctx::Effects, Ctx::Variables)>()
        }
    }
}

#[macro_export]
macro_rules! ctx_if_construct {
    {
        predicate = (set $var:ty)
        where = ()
        then = ($then:expr)
        else = ($else:expr)
    }=> {
        $crate::conditional::IfAction::<$crate::conditional::IsVariableIn::<$var>, _, _>::new($then, $else)
    };
    {
        predicate = ( $cond:expr )
        where = ($($bind:ident <- $var:ty),*)
        then = ($then:expr)
        else = ($else:expr)
    } => {{
        #[doc(hidden)]
        struct __Condition;

        #[doc(hidden)]
        impl $crate::conditional::Condition for __Condition {
            type Bool<__Ctx: $crate::action::ActionContext> = __ConditionBool<__Ctx>;
        }

        #[doc(hidden)]
        struct __ConditionBool<__Ctx: $crate::action::ActionContext>(::core::marker::PhantomData<__Ctx>);

        #[doc(hidden)]
        impl<__Ctx: $crate::action::ActionContext> $crate::value::ConstValue for __ConditionBool<__Ctx> {
            type Type = bool;
            const VALUE: Self::Type = {
                $(let $bind = $crate::variable::list::find_variable::<__Ctx::Variables, $var>();)*
                $cond
            };
        }

        $crate::conditional::IfAction::<__Condition, _, _>::new($then, $else)
    }}
}

#[macro_export]
macro_rules! ctx_if_else {
    {
        predicate = ($($predicate:tt)*)
        where = ($($binding:tt)*)
        then = ($($then:tt)*)
        else = ($($else:tt)*)
        rest = ()
    } => {
        $crate::ctx_if_construct! {
            predicate = ($($predicate)*)
            where = ($($binding)*)
            then = ($($then)*)
            else = ($($else)*)
        }
    };
    {
        predicate = ($($predicate:tt)*)
        where = ($($binding:tt)*)
        then = ($($then:tt)*)
        else = ($($else:tt)*)
        rest = ($e:tt $($rest:tt)*)
    } => {
        $crate::ctx_if_else! {
            predicate = ($($predicate)*)
            where = ($($binding)*)
            then = ($($then)*)
            else = ($($else)* $e)
            rest = ($($rest)*)
        }
    };
}

#[macro_export]
macro_rules! ctx_if_then {
    {
        predicate = ($($predicate:tt)*)
        where = ($($binding:tt)*)
        then = ($($then:tt)*)
        rest = (else $($rest:tt)*)
    } => {
        $crate::ctx_if_else! {
            predicate = ($($predicate)*)
            where = ($($binding)*)
            then = ($($then)*)
            else = ()
            rest = ($($rest)*)
        }
    };
    {
        predicate = ($($predicate:tt)*)
        where = ($($binding:tt)*)
        then = ($($then:tt)*)
        rest = ($e:tt $($rest:tt)*)
    } => {
        $crate::ctx_if_then! {
            predicate = ($($predicate)*)
            where = ($($binding)*)
            then = ($($then)* $e)
            rest = ($($rest)*)
        }
    };
}

#[macro_export]
macro_rules! ctx_if_where {
    {
        predicate = ($($predicate:tt)*)
        where = ($($binding:tt)*)
        rest = (then $($rest:tt)*)
    } => {
        $crate::ctx_if_then! {
            predicate = ($($predicate)*)
            where = ($($binding)*)
            then = ()
            rest = ($($rest)*)
        }
    };
    {
        predicate = ($($predicate:tt)*)
        where = ($($binding:tt)*)
        rest = ($where:tt $($rest:tt)*)
    } => {
        $crate::ctx_if_where! {
            predicate = ($($predicate)*)
            where = ($($binding)* $where)
            rest = ($($rest)*)
        }
    };
}

#[macro_export]
macro_rules! ctx_if_predicate {
    {
        predicate = ($($predicate:tt)*)
        rest = (where $($rest:tt)*)
    } => {
        $crate::ctx_if_where! {
            predicate = ($($predicate)*)
            where = ()
            rest = ($($rest)*)
        }
    };
    {
        predicate = ($($predicate:tt)*)
        rest = (then $($rest:tt)*)
    } => {
        $crate::ctx_if_then! {
            predicate = ($($predicate)*)
            where = ()
            then = ()
            rest = ($($rest)*)
        }
    };
    {
        predicate = ($($predicate:tt)*)
        rest = ($cond:tt $($rest:tt)*)
    } => {
        $crate::ctx_if_predicate! {
            predicate = ($($predicate)* $cond)
            rest = ($($rest)*)
        }
    };
}

#[macro_export]
macro_rules! ctx_if {
    (if set $($rest:tt)*) => {
        $crate::ctx_if_predicate! {
            predicate = (set)
            rest = ($($rest)*)
        }
    };
    (if $($rest:tt)*) => {
        $crate::ctx_if_predicate! {
            predicate = ()
            rest = ($($rest)*)
        }
    };
}

#[test]
#[cfg(test)]
fn test() {
    use crate::ctx;

    type Var = (u32, u32);
    type Var2 = (u64, u64);

    let action = ctx! {
        set Var = 45;
        ctx_if!(
            if a + b == 90 where a <- Var, b <- Var then
                ctx! { pure "==" }
            else
                ctx! { pure "!=" }
        )
    };
    assert_eq!(action.run(), "==");

    let action = ctx! {
        set Var = 45;
        ctx_if!(
            if a + b == 90 where a <- Var, b <- Var then
                ctx! { set Var2 = 42; }
            else
                ctx! { }
        );
        get Var2
    };
    assert_eq!(action.run(), 42);

    let action = ctx! {
        set Var = 45;
        ctx_if!(
            if set Var then
                ctx! { set Var2 = 42; }
            else
                ctx! { panic "Var doesn't exist."; }
        );
        get Var2
    };
    assert_eq!(action.run(), 42);
}
