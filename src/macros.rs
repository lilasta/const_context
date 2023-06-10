#[macro_export]
macro_rules! ctx {
    { $($rest:tt)* } => {
        $crate::action::lazy::LazyAction::new(move || $crate::ctx_parse! {
            action = ()
            rest = ($($rest)*)
        })
    }
}

#[macro_export]
macro_rules! ctx_parse {
    {
        action = ()
        rest = ()
    } => {
        $crate::ctx_action!()
    };
    {
        action = ($($action:tt)*)
        rest = ()
    } => {
        $crate::ctx_action!($($action)*)
    };
    {
        action = (_ <- $($action:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::ctx_action!($($action)*),
            move |_| $crate::ctx_parse! {
                action = ()
                rest = ($($rest)*)
            },
        )
    };
    {
        action = ($var:ident <- $($action:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::ctx_action!($($action)*),
            move |$var| $crate::ctx_parse! {
                action = ()
                rest = ($($rest)*)
            },
        )
    };
    {
        action = (let _ $(: $ty:ty)? = $($e:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::action::pure::PureAction::new($($e)*),
            move |_ $(: $ty)?| $crate::ctx_parse! {
                action = ()
                rest = ($($rest)*)
            },
        )
    };
    {
        action = (let $var:ident $(: $ty:ty)? = $($e:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::action::pure::PureAction::new($($e)*),
            move |$var $(: $ty)?| $crate::ctx_parse! {
                action = ()
                rest = ($($rest)*)
            },
        )
    };
    {
        action = (const _ : $ty:ty = $e:expr)
        rest = (; $($rest:tt)*)
    } => {{
        const _ : $ty = $e;
        $crate::ctx_parse! {
            action = ()
            rest = ($($rest)*)
        }
    }};
    {
        action = (const $name:ident : $ty:ty = $e:expr)
        rest = (; $($rest:tt)*)
    } => {{
        const $name : $ty = $e;
        $crate::ctx_parse! {
            action = ()
            rest = ($($rest)*)
        }
    }};
    {
        action = (type $name:ident = $ty:ty)
        rest = (; $($rest:tt)*)
    } => {{
        type $name = $ty;
        $crate::ctx_parse! {
            action = ()
            rest = ($($rest)*)
        }
    }};
    {
        action = ($($action:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::ctx_action!($($action)*),
            move |_| $crate::ctx_parse! {
                action = ()
                rest = ($($rest)*)
            },
        )
    };
    {
        action = ($($action:tt)*)
        rest = ($other:tt $($rest:tt)*)
    } => {
        $crate::ctx_parse! {
            action = ($($action)* $other)
            rest = ($($rest)*)
        }
    }
}

#[macro_export]
macro_rules! ctx_action {
    () => {
        $crate::action::pure::PureAction::new(())
    };
    (pure $e:expr) => {
        $crate::action::pure::PureAction::new($e)
    };
    (get $cvar:ty) => {
        $crate::action::variable_get::GetAction::<$cvar>::new()
    };
    (set $var:ty = $e:expr) => {
        $crate::action::variable_set::SetAction::<$var, { $crate::value::ConstValue::new::<<$var as $crate::variable::ConstVariable>::Value>($e) }>::new()
    };
    (set $($rest:tt)*) => {
        $crate::ctx_set! {
            state = parse_dst
            rest = [ $($rest)* ]
        }
    };
    (unset $cvar:ty) => {
        $crate::action::variable_unset::UnsetAction::<$cvar>::new()
    };
    (get effect $f:ty) => {
        $crate::action::effect_get::GetEffectAction::<$f>::new()
    };
    (effect $f:ty = $fc:expr) => {
        $crate::action::effect_set::SetEffectAction::<$f, _>::new($fc)
    };
    (panic $msg:expr) => {
        $crate::action::panic::PanicAction::<{ $msg }, _>::new()
    };
    ($action:expr) => {
        $action
    };
}
