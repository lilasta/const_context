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
            isset = []
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
            isset = []
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ $name:ident <- set? $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_isset_var
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            isset = [ $($isset)* $name <- ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        state = parse_isset_var
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* , ]
            effects = [ $($effect)* ]
            generic_names = [ $($generic_name)* ]
            generic_bounds = [ $($generic_bound)* ]
            generic_params = [ $($generic_param)* ]
            generic_const = [ $($generic_const)* ]
            generic_const_params = [ $($generic_const_param)* ]
        }
    };
    {
        state = parse_isset_var
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* , ]
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
        state = parse_isset_var
        dst = [ $($dst:tt)* ]
        expr = [ $($expr:tt)* ]
        binds = [ $($bind:tt)* ]
        isset = [ $($isset:tt)* ]
        effects = [ $($effect:tt)* ]
        generic_names = [ $($generic_name:tt)* ]
        generic_bounds = [ $($generic_bound:tt)* ]
        generic_params = [ $($generic_param:tt)* ]
        generic_const = [ $($generic_const:tt)* ]
        generic_const_params = [ $($generic_const_param:tt)* ]
        rest = [ $other:tt $($rest:tt)* ]
    } => {
        $crate::ctx_set! {
            state = parse_isset_var
            dst = [ $($dst)* ]
            expr = [ $($expr)* ]
            binds = [ $($bind)* ]
            isset = [ $($isset)* $other ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:tt)* ]
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
            isset = [ $($isset)* ]
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
        isset = [ $($isset:ident <- $isset_var:ty,)* ]
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
        struct __CustomConstValue
            <__Ctx: $crate::context::ConstContext, $($generic_name,)* $(const $generic_const : $generic_const_type,)*>
            (::core::marker::PhantomData<(__Ctx, $($generic_name,)*)>);

        #[doc(hidden)]
        impl<__Ctx, $($generic_name,)* $(const $generic_const : $generic_const_type,)*> $crate::value::ConstValue
        for __CustomConstValue<__Ctx, $($generic_name,)* $($generic_const,)*>
        where
            __Ctx: $crate::context::ConstContext,
            $($generic_bound)*
        {
            type Type = <$dst as $crate::variable::Variable>::Type;
            const VALUE: Self::Type = {
                $(let $bind = $crate::variable::list::find_variable::<__Ctx::Variables, $from>();)*
                $(let $isset = $crate::variable::list::is_variable_in::<__Ctx::Variables, $isset_var>();)*
                $(let $bind_eff = $crate::effect::get_const::<__Ctx::Effects, $eff>();)*
                $expr
            };
        }

        #[doc(hidden)]
        impl<$($generic_name,)* $(const $generic_const : $generic_const_type,)*> $crate::action::Action
        for __CustomSetAction<$($generic_name,)* $($generic_const,)*>
        where
            $($generic_bound)*
        {
            type Output = ();
            type Context<__Ctx: $crate::context::ConstContext> = (
                __Ctx::Strictness,
                __Ctx::Effects,
                $crate::variable::list::VariableListCons<$dst, __CustomConstValue<__Ctx, $($generic_name,)* $($generic_const,)*>, __Ctx::Variables>
            );

            #[inline(always)]
            fn run_with<__Ctx: $crate::context::ConstContext>(self) -> Self::Output {
                $crate::value::strict_if::<__Ctx::Strictness, __CustomConstValue<__Ctx, $($generic_name,)* $($generic_const,)*>>();
            }
        }

        __CustomSetAction::<$($generic_param,)* $({ $generic_const_param },)*>(::core::marker::PhantomData)
    }};
}
