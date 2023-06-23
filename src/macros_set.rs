#[macro_export]
macro_rules! ctx_set {
    {
        state = parse_dst
        rest = [ $dst:ty = $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_expr
            dst = [ $dst ]
            expr = []
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_expr
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        rest = []
    } => {
        $crate::ctx_set! {
            state = construct
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = []
            effects = []
            generic_names = []
            generic_bounds = []
            generic_params = []
            generic_const = []
            generic_const_params = []
        }
    };
    {
        state = parse_expr
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        rest = [ where $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_where
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = []
            effects = []
            generic_names = []
            generic_bounds = []
            generic_params = []
            generic_const = []
            generic_const_params = []
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_expr
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        rest = [ $other:tt $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_expr
            dst = [ $($dst)* ]
            expr = [ $($expr)* $other ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_where
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = []
    } => {
        $crate::ctx_set! {
            state = construct
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
        }
    };
    {
        state = parse_where
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ $name:ident <- get $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_bind_from
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* $name <- ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_where
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ $name:ident <- effect $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_effect_from
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* $name <- ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_where
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ const $name:ident : $ty:ty = $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_const_param
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* $name : $ty, ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_where
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ $name:ident $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_generic
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* $name, ]
            generic_bounds = [ $($generic_bound)* $name : ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_bind_from
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = []
    } => {
        $crate::ctx_set! {
            state = construct
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* , ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
        }
    };
    {
        state = parse_bind_from
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ , $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_where
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* , ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_bind_from
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ $other:tt $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_bind_from
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* $other ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_effect_from
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = []
    } => {
        $crate::ctx_set! {
            state = construct
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* , ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
        }
    };
    {
        state = parse_effect_from
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ , $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_where
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* , ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_effect_from
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ $other:tt $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_effect_from
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* $other ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_const_param
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = []
    } => {
        $crate::ctx_set! {
            state = construct
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* , ]
        }
    };
    {
        state = parse_const_param
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ , $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_where
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* , ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_const_param
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ $other:tt $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_const_param
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* $other ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_generic
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ = $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_generic_param
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* , ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_generic
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ : $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_generic_bound
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_generic_bound
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ = $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_generic_param
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* , ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_generic_bound
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ $other:tt $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_generic_bound
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* $other ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_generic_param
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = []
    } => {
        $crate::ctx_set! {
            state = construct
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* , ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
        }
    };
    {
        state = parse_generic_param
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ , $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_where
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* , ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = parse_generic_param
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ $other:tt $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_generic_param
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* $other ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
            rest = [ $($rest)* ]
        }
    };
    {
        state = construct
        dst = [ $dst:ty ]
        expr = [ $expr:expr ]
        binds = [ $($bind:ident <- $from:ty,)* ]
        effects = [ $($bind_eff:ident <- $eff:ty,)* ]
        generic_names = [ $($generic_name:ident,)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:ty,)* ]
        generic_const = [ $($generic_const:ident : $generic_const_type:ty,)* ]
        generic_const_params = [ $($generic_const_param:expr,)* ]
    } => {{
        #[doc(hidden)]
        #[allow(unused_parens)]
        struct __CustomSetAction
            <$($generic_name,)* $(const $generic_const : $generic_const_type,)*>
            (::core::marker::PhantomData<($($generic_name,)*)>);

        #[doc(hidden)]
        #[allow(unused_parens)]
        struct __CustomVariableList
            <Ctx: $crate::action::ActionContext, $($generic_name,)* $(const $generic_const : $generic_const_type,)*>
            (::core::marker::PhantomData<(Ctx, $($generic_name,)*)>);

        #[doc(hidden)]
        impl<Ctx, $($generic_name,)* $(const $generic_const : $generic_const_type,)*> $crate::variable::VariableList
        for __CustomVariableList<Ctx, $($generic_name,)* $($generic_const,)*>
        where
            Ctx: $crate::action::ActionContext,
            $($generic_bound)*
        {
            type Next = Ctx::Variables;
            type Variable = $dst;
            const VALUE: $crate::variable::VariableListValue<<$dst as $crate::variable::ConstVariable>::Value> = $crate::variable::VariableListValue::Has({
                $(let $bind = $crate::variable::find_variable::<Ctx::Variables, $from>();)*
                $(let $bind_eff = $crate::effect::get_const::<Ctx::Effects, $eff>();)*
                $expr
            });
        }

        #[doc(hidden)]
        impl<$($generic_name,)* $(const $generic_const : $generic_const_type,)*> $crate::action::Action
        for __CustomSetAction<$($generic_name,)* $($generic_const,)*>
        where
            $($generic_bound)*
        {
            type Output = ();
            type Context<Ctx: $crate::action::ActionContext> = (Ctx::Effects, __CustomVariableList<Ctx, $($generic_name,)* $($generic_const,)*>);

            #[inline(always)]
            fn eval<Ctx: $crate::action::ActionContext>(self) -> Self::Output {}
        }

        __CustomSetAction::<$($generic_param,)* $({ $generic_const_param },)*>(::core::marker::PhantomData)
    }};
}
