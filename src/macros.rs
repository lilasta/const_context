#[macro_export]
macro_rules! ctx {
    { $($rest:tt)* } => {
        $crate::action::lazy::LazyAction::new(move || $crate::ctx_parse! {
            action = ()
            binds = ()
            rest = ($($rest)*)
        })
    }
}

#[macro_export]
macro_rules! ctx_parse {
    {
        action = ()
        binds = ($($binds:tt)*)
        rest = ()
    } => {
        $crate::ctx_action!()
    };
    {
        action = ($($action:tt)*)
        binds = ($($binds:tt)*)
        rest = ()
    } => {
        $crate::ctx_action!($($action)*)
    };
    {
        action = (_ <- move $name:ident)
        binds = ($($binds:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::action::pure::PureAction::new($name),
            {
                // TODO: Find a way to remove the token
                #[doc(hidden)]
                struct __Moved;
                let $name = __Moved;
                $crate::ctx_rtc_pack!($($binds)*)
            },
            #[allow(unused_variables)]
            move |_, __rt_ctx| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = (_ <- $($action:tt)*)
        binds = ($($binds:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::ctx_action!($($action)*),
            $crate::ctx_rtc_pack!($($binds)*),
            #[allow(unused_variables)]
            move |_, __rt_ctx| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = ($var:ident <- move $name:ident)
        binds = ($($binds:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::action::pure::PureAction::new($name),
            {
                // TODO: Find a way to remove the token
                #[doc(hidden)]
                struct __Moved;
                let $name = __Moved;
                $crate::ctx_rtc_pack!($($binds)*)
            },
            #[allow(unused_variables)]
            move |$var, __rt_ctx| {
                let __tmp = $var;
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*);
                let $var = __tmp;
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = ($var:ident <- $($action:tt)*)
        binds = ($($binds:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::ctx_action!($($action)*),
            $crate::ctx_rtc_pack!($($binds)*),
            #[allow(unused_variables)]
            move |$var, __rt_ctx| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($var $($binds)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = (let _ $(: $ty:ty)? = $($e:tt)*)
        binds = ($($binds:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::action::pure::PureAction::new($($e)*),
            $crate::ctx_rtc_pack!($($binds)*),
            #[allow(unused_variables)]
            move |_ $(: $ty)?, __rt_ctx| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = (let $var:ident $(: $ty:ty)? = $($e:tt)*)
        binds = ($($binds:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::action::pure::PureAction::new($($e)*),
            $crate::ctx_rtc_pack!($($binds)*),
            #[allow(unused_variables)]
            move |$var $(: $ty)?, __rt_ctx| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($var $($binds)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = (const _ : $ty:ty = $e:expr)
        binds = ($($binds:tt)*)
        rest = (; $($rest:tt)*)
    } => {{
        const _ : $ty = $e;
        $crate::ctx_parse! {
            action = ()
            binds = ($($binds)*)
            rest = ($($rest)*)
        }
    }};
    {
        action = (const $name:ident : $ty:ty = $e:expr)
        binds = ($($binds:tt)*)
        rest = (; $($rest:tt)*)
    } => {{
        const $name : $ty = $e;
        $crate::ctx_parse! {
            action = ()
            binds = ($($binds)*)
            rest = ($($rest)*)
        }
    }};
    {
        action = (type $name:ident = $ty:ty)
        binds = ($($binds:tt)*)
        rest = (; $($rest:tt)*)
    } => {{
        type $name = $ty;
        $crate::ctx_parse! {
            action = ()
            binds = ($($binds)*)
            rest = ($($rest)*)
        }
    }};
    {
        action = ($($action:tt)*)
        binds = ($($binds:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::ctx_action!($($action)*),
            $crate::ctx_rtc_pack!($($binds)*),
            #[allow(unused_variables)]
            move |_, __rt_ctx| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = ($($action:tt)*)
        binds = ($($binds:tt)*)
        rest = ($other:tt $($rest:tt)*)
    } => {
        $crate::ctx_parse! {
            action = ($($action)* $other)
            binds = ($($binds)*)
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
    (set $var:ty = $e:expr) => {{
        #[doc(hidden)]
        struct __ConstValue;

        #[doc(hidden)]
        impl $crate::value::ConstValue for __ConstValue {
            type Type = <$var as $crate::variable::Variable>::Type;
            const VALUE: Self::Type = $e;
        }

        $crate::action::variable_set::SetAction::<$var, __ConstValue>::new()
    }};
    (set $($rest:tt)*) => {
        $crate::ctx_set! {
            state = parse_dst
            rest = [ $($rest)* ]
        }
    };
    (unset $cvar:ty) => {
        $crate::action::variable_unset::UnsetAction::<$cvar>::new()
    };
    (effect $f:ty = $fc:expr) => {
        $crate::action::effect_set::SetEffectAction::<$f, _>::new($fc)
    };
    (effect $f:ty) => {
        $crate::action::effect_get::GetEffectAction::<$f>::new()
    };
    (panic $msg:expr) => {{
        #[doc(hidden)]
        struct __PanicMsg;

        #[doc(hidden)]
        impl $crate::action::panic::PanicMessage for __PanicMsg {
            const MSG: &'static str = $msg;
        }

        $crate::action::panic::PanicAction::<__PanicMsg, _>::new()
    }};
    ($action:expr) => {
        $action
    };
}

#[macro_export]
macro_rules! ctx_rtc_pack {
    ($($name:ident)*) => {{
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        struct __RuntimeContext<$($name),*> {
            $($name : $name),*
        }

        __RuntimeContext {
            $($name),*
        }
    }};
}

#[macro_export]
macro_rules! ctx_rtc_unpack {
    ($ctx:ident, ) => {};
    ($ctx:ident, $name:ident $($rest:tt)*) => {
        let $name = $ctx.$name;
        $crate::ctx_rtc_unpack!($ctx, $($rest)*)
    };
}
