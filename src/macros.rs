#[macro_export]
macro_rules! ctx {
    { $($rest:tt)* } => {
        $crate::action::lazy::LazyAction::new(
            #[inline(always)]
            move || $crate::action::map::MapAction::new(
                $crate::ctx_parse! {
                    action = ()
                    binds = ()
                    rest = ($($rest)*)
                },
                #[inline(always)]
                |(__ret, _)| __ret,
            )
        )
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! ctx_parse {
    {
        action = (if get $var:ty)
        binds = ($($binds:tt)*)
        rest = ($($rest:tt)*)
    } => {
        $crate::ctx_parse! {
            action = (if $crate::condition::GetBool<$var>)
            binds = ($($binds)*)
            rest = ($($rest)*)
        }
    };
    {
        action = (if set? $var:ty)
        binds = ($($binds:tt)*)
        rest = ($($rest:tt)*)
    } => {
        $crate::ctx_parse! {
            action = (if $crate::condition::IsSet<$var>)
            binds = ($($binds)*)
            rest = ($($rest)*)
        }
    };
    {
        action = (if $var:ty { $($then:tt)* } else { $($else:tt)* })
        binds = ($($binds:tt)*)
        rest = ($($rest:tt)*)
    } => {{
        $crate::action::r#if::IfAction::<$var, _, _, _>::new(
            $crate::ctx_rtc_pack!($($binds)*),
            #[allow(unused_variables)]
            #[inline(always)]
            move |__rt_ctx| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    rest = ($($then)*; $($rest)*)
                }
            },
            #[allow(unused_variables)]
            #[inline(always)]
            move |__rt_ctx| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    rest = ($($else)*; $($rest)*)
                }
            },
        )
    }};
    {
        action = (_ <- move $name:ident)
        binds = ($($binds:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            // TODO: Find a way to remove the token
            $crate::action::map::MapAction::new(
                $crate::ctx_action!(pure $name),
                move |__value| {
                    #[doc(hidden)]
                    struct __Moved;
                    let $name = __Moved;
                    (__value, $crate::ctx_rtc_pack!($($binds)*))
                }
            ),
            #[allow(unused_variables)]
            #[inline(always)]
            move |(_, __rt_ctx)| {
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
            $crate::ctx_parse! {
                action = ($($action)*)
                binds = ($($binds)*)
                rest = ()
            },
            #[allow(unused_variables)]
            #[inline(always)]
            move |(_, __rt_ctx)| {
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
            // TODO: Find a way to remove the token
            $crate::action::map::MapAction::new(
                $crate::ctx_action!(pure $name),
                move |__value| {
                    #[doc(hidden)]
                    struct __Moved;
                    let $name = __Moved;
                    (__value, $crate::ctx_rtc_pack!($($binds)*))
                }
            ),
            #[allow(unused_variables)]
            #[inline(always)]
            move |($var, __rt_ctx)| {
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
            $crate::ctx_parse! {
                action = ($($action)*)
                binds = ($($binds)*)
                rest = ()
            },
            #[allow(unused_variables)]
            #[inline(always)]
            move |($var, __rt_ctx)| {
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
            $crate::ctx_parse! {
                action = (pure $($e)*)
                binds = ($($binds)*)
                rest = ()
            },
            #[allow(unused_variables)]
            #[inline(always)]
            move |(_, __rt_ctx) $(: ($ty, _))?| {
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
            $crate::ctx_parse! {
                action = (pure $($e)*)
                binds = ($($binds)*)
                rest = ()
            },
            #[allow(unused_variables)]
            #[inline(always)]
            move |($var, __rt_ctx) $(: ($ty, _))?| {
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
        $crate::ctx_parse! {
            action = (_ <- $($action)*)
            binds = ($($binds)*)
            rest = (; $($rest)*)
        }
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
    };
    {
        action = ($($action:tt)*)
        binds = ($($binds:tt)*)
        rest = ()
    } => {
        $crate::action::map::MapAction::new(
            $crate::ctx_action!($($action)*),
            move |__value| (__value, $crate::ctx_rtc_pack!($($binds)*)),
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! ctx_action {
    () => {
        $crate::action::pure::PureAction::new(())
    };
    (pure $e:expr) => {
        $crate::action::pure::PureAction::new($e)
    };
    (get $var:ty) => {
        $crate::action::variable_get::GetAction::<$var>::new()
    };
    (set? $var:ty) => {
        $crate::action::variable_isset::IsSetAction::<$var>::new()
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
    (panic $msg:expr) => {
        // This will be strictly evaluated, so it will cause a compile error.
        $crate::ctx_action!(set () = panic!($msg))
    };
    ($action:expr) => {
        $action
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! ctx_rtc_pack {
    () => {
        ()
    };
    ($name:ident $($rest:ident)*) => {
        ($name, $crate::ctx_rtc_pack!($($rest)*))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! ctx_rtc_unpack {
    ($ctx:expr,) => {};
    ($ctx:expr, $name:ident $($rest:tt)*) => {
        let $name = $ctx.0;
        $crate::ctx_rtc_unpack!($ctx.1, $($rest)*)
    };
}
