pub mod bool;

use core::marker::PhantomData;

use crate::action::{Action, ActionContext};
use crate::conditional::bool::{ConstBool, ConstBoolToTypeBool, False, True, TypeBool};
use crate::variable::list::{is_variable_in, VariableList};
use crate::variable::Variable;

pub trait Condition {
    type Bool<Ctx: ActionContext>: TypeBool;
}

pub struct IsVariableIn<Var: Variable>(PhantomData<Var>);

impl<Var: Variable> Condition for IsVariableIn<Var> {
    type Bool<Ctx: ActionContext> =
        <ValueOfIsVariableIn<Var, Ctx::Variables> as ConstBoolToTypeBool>::Type;
}

pub struct ValueOfIsVariableIn<Var: Variable, Vars: VariableList>(PhantomData<(Var, Vars)>);

impl<Var: Variable, Vars: VariableList> ConstBool for ValueOfIsVariableIn<Var, Vars> {
    const BOOL: bool = is_variable_in::<Vars, Var>();
}

pub trait If<Output> {
    type Action: Action<Output = Output>;
    fn then(self) -> Self::Action;
}

pub struct ConcreteIf<Cond, A, B>(A, B, PhantomData<Cond>);

impl<Cond, A, B, Output> ConcreteIf<Cond, A, B>
where
    Cond: TypeBool,
    A: Action<Output = Output>,
    B: Action<Output = Output>,
{
    #[inline(always)]
    pub const fn new(a: A, b: B) -> Self {
        Self(a, b, PhantomData)
    }
}

impl<Cond, A, B, Output> If<Output> for ConcreteIf<Cond, A, B>
where
    A: Action<Output = Output>,
    B: Action<Output = Output>,
    Cond: TypeBool,
{
    default type Action = B;
    default fn then(self) -> Self::Action {
        unreachable!()
    }
}

impl<A, B, Output> If<Output> for ConcreteIf<False, A, B>
where
    A: Action<Output = Output>,
    B: Action<Output = Output>,
{
    type Action = B;

    #[inline(always)]
    fn then(self) -> Self::Action {
        self.1
    }
}

impl<A, B, Output> If<Output> for ConcreteIf<True, A, B>
where
    A: Action<Output = Output>,
    B: Action<Output = Output>,
{
    type Action = A;

    #[inline(always)]
    fn then(self) -> Self::Action {
        self.0
    }
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
    type Context<Ctx: ActionContext> =
        <<ConcreteIf<Cond::Bool<Ctx>, A, B> as If<A::Output>>::Action as Action>::Context<Ctx>;

    #[inline(always)]
    fn run_with<Ctx: ActionContext>(self) -> Self::Output {
        let Self(a, b, ..) = self;
        ConcreteIf::<Cond::Bool<Ctx>, A, B>::new(a, b)
            .then()
            .run_with::<Ctx>()
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
            type Bool<Ctx: $crate::action::ActionContext> = <__ConditionBool<Ctx> as $crate::conditional::bool::ConstBoolToTypeBool>::Type;
        }

        #[doc(hidden)]
        struct __ConditionBool<Ctx: $crate::action::ActionContext>(::core::marker::PhantomData<Ctx>);

        #[doc(hidden)]
        impl<Ctx: $crate::action::ActionContext> $crate::conditional::ConstBool for __ConditionBool<Ctx> {
            const BOOL: bool = {
                $(let $bind = $crate::variable::list::find_variable::<Ctx::Variables, $var>();)*
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
