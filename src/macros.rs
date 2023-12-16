#[doc(hidden)]
pub struct __Moved;

#[macro_export]
macro_rules! ctx {
    { $($rest:tt)* } => {
        $crate::action::lazy::LazyAction::new(
            #[inline(always)]
            move || $crate::action::map::MapAction::new(
                $crate::ctx_parse! {
                    action = ()
                    binds = ()
                    binds_log = ()
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
        action = (_ <- if [ $cond:ty ] { $($then:tt)* } else { $($else:tt)* })
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = ($($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::ctx_if! {
                condition = ($cond)
                binds = ($($binds)*)
                binds_log = ($($binds_log)*)
                then = ($($then)*)
                else = ($($else)*)
            },
            #[allow(unused_variables)]
            #[inline(always)]
            move |(_, __rt_ctx)| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*, $($binds_log)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    binds_log = ($($binds_log)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = ($name:ident <- if [ $cond:ty ] { $($then:tt)* } else { $($else:tt)* })
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = ($($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::ctx_if! {
                condition = ($cond)
                binds = ($($binds)*)
                binds_log = ($($binds_log)*)
                then = ($($then)*)
                else = ($($else)*)
            },
            #[allow(unused_variables)]
            #[inline(always)]
            move |($name, __rt_ctx)| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*, $($binds_log)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($name $($binds)*)
                    binds_log = (let $name $($binds_log)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = (if [ $cond:ty ] { $($then:tt)* } else { $($else:tt)* })
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = ($($rest:tt)*)
    } => {
        $crate::ctx_parse! {
            action = (_ <- if [ $cond ] { $($then)* } else { $($else)* })
            binds = ($($binds)*)
            binds_log = ($($binds_log)*)
            rest = ($($rest)*)
        }
    };
    {
        action = (_ <- move $name:ident)
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            // TODO: Find a way to remove the token
            $crate::action::map::MapAction::new(
                $crate::ctx_action!(pure $name),
                #[inline(always)]
                move |__value| {
                    #[doc(hidden)]
                    struct __Moved;
                    let $name = __Moved;
                    (__value, $crate::ctx_rtc_pack!($($binds)*, $($binds_log)*))
                }
            ),
            #[allow(unused_variables)]
            #[inline(always)]
            move |(_, __rt_ctx)| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*, $($binds_log)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    binds_log = (move $name $($binds_log)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = (_ <- $($action:tt)*)
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::ctx_parse! {
                action = ($($action)*)
                binds = ($($binds)*)
                binds_log = ($($binds_log)*)
                rest = ()
            },
            #[allow(unused_variables)]
            #[inline(always)]
            move |(_, __rt_ctx)| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*, $($binds_log)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    binds_log = ($($binds_log)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = ($var:ident <- move $name:ident)
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            // TODO: Find a way to remove the token
            $crate::action::map::MapAction::new(
                $crate::ctx_action!(pure $name),
                #[inline(always)]
                move |__value| {
                    #[doc(hidden)]
                    struct __Moved;
                    let $name = __Moved;
                    (__value, $crate::ctx_rtc_pack!($($binds)*, $($binds_log)*))
                }
            ),
            #[allow(unused_variables)]
            #[inline(always)]
            move |($var, __rt_ctx)| {
                let __tmp = $var;
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*, $($binds_log)*);
                let $var = __tmp;
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    binds_log = (move $name $($binds_log)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = ($var:ident <- $($action:tt)*)
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::ctx_parse! {
                action = ($($action)*)
                binds = ($($binds)*)
                binds_log = ($($binds_log)*)
                rest = ()
            },
            #[allow(unused_variables)]
            #[inline(always)]
            move |($var, __rt_ctx)| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*, $($binds_log)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($var $($binds)*)
                    binds_log = (let $var $($binds_log)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = (let _ $(: $ty:ty)? = $($e:tt)*)
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::ctx_parse! {
                action = (pure $($e)*)
                binds = ($($binds)*)
                binds_log = ($($binds_log)*)
                rest = ()
            },
            #[allow(unused_variables)]
            #[inline(always)]
            move |(_, __rt_ctx) $(: ($ty, _))?| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*, $($binds_log)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    binds_log = ($($binds_log)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = (let $var:ident $(: $ty:ty)? = $($e:tt)*)
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::action::bind::BindAction::new(
            $crate::ctx_parse! {
                action = (pure $($e)*)
                binds = ($($binds)*)
                binds_log = ($($binds_log)*)
                rest = ()
            },
            #[allow(unused_variables)]
            #[inline(always)]
            move |($var, __rt_ctx) $(: ($ty, _))?| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*, $($binds_log)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($var $($binds)*)
                    binds_log = (let $var $($binds_log)*)
                    rest = ($($rest)*)
                }
            },
        )
    };
    {
        action = (const _ : $ty:ty = $e:expr)
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = (; $($rest:tt)*)
    } => {{
        const _ : $ty = $e;
        $crate::ctx_parse! {
            action = ()
            binds = ($($binds)*)
            binds_log = ($($binds_log)*)
            rest = ($($rest)*)
        }
    }};
    {
        action = (const $name:ident : $ty:ty = $e:expr)
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = (; $($rest:tt)*)
    } => {{
        const $name : $ty = $e;
        $crate::ctx_parse! {
            action = ()
            binds = ($($binds)*)
            binds_log = ($($binds_log)*)
            rest = ($($rest)*)
        }
    }};
    {
        action = (type $name:ident = $ty:ty)
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = (; $($rest:tt)*)
    } => {{
        type $name = $ty;
        $crate::ctx_parse! {
            action = ()
            binds = ($($binds)*)
            binds_log = ($($binds_log)*)
            rest = ($($rest)*)
        }
    }};
    {
        action = ($($action:tt)*)
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = (; $($rest:tt)*)
    } => {
        $crate::ctx_parse! {
            action = (_ <- $($action)*)
            binds = ($($binds)*)
            binds_log = ($($binds_log)*)
            rest = (; $($rest)*)
        }
    };
    {
        action = ($($action:tt)*)
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = ($other:tt $($rest:tt)*)
    } => {
        $crate::ctx_parse! {
            action = ($($action)* $other)
            binds = ($($binds)*)
            binds_log = ($($binds_log)*)
            rest = ($($rest)*)
        }
    };
    {
        action = ($($action:tt)*)
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        rest = ()
    } => {
        $crate::action::map::MapAction::new(
            $crate::ctx_action!($($action)*),
            #[inline(always)]
            move |__value| (__value, $crate::ctx_rtc_pack!($($binds)*, $($binds_log)*)),
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! ctx_if {
    {
        condition = ($($cond:tt)*)
        binds = ($($binds:tt)*)
        binds_log = ($($binds_log:tt)*)
        then = ($($then:tt)*)
        else = ($($else:tt)*)
    } => {
        $crate::action::r#if::IfAction::<$($cond)*, _, _, _>::new(
            $crate::ctx_rtc_pack!($($binds)*, $($binds_log)*),
            #[allow(unused_variables)]
            #[inline(always)]
            move |__rt_ctx| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*, $($binds_log)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    binds_log = ($($binds_log)*)
                    rest = ($($then)*)
                }
            },
            #[allow(unused_variables)]
            #[inline(always)]
            move |__rt_ctx| {
                $crate::ctx_rtc_unpack!(__rt_ctx, $($binds)*, $($binds_log)*);
                $crate::ctx_parse! {
                    action = ()
                    binds = ($($binds)*)
                    binds_log = ($($binds_log)*)
                    rest = ($($else)*)
                }
            },
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
    ($($binds:ident)*, $($log:tt)*) => {
        $crate::ctx_rtc_pack_check! {
            binds = ($($binds)*)
            binds_log_checking = ($($log)*)
            binds_log_original = ($($log)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! ctx_rtc_pack_check {
    {
        binds = ()
        binds_log_checking = ($($log:tt)*)
        binds_log_original = ($($log_orig:tt)*)
    } => {
        ()
    };
    {
        binds = ($name:ident $($rest:ident)*)
        binds_log_checking = ()
        binds_log_original = ($($log_orig:tt)*)
     } => {
        ::core::compile_error!("")
    };
    {
        binds = ($name:ident $($rest:ident)*)
        binds_log_checking = (let $name2:ident $($log:tt)*)
        binds_log_original = ($($log_orig:tt)*)
    } => {{
        #[doc(hidden)]
        macro_rules! __if_same_ident {
            ($name $name) => {
                ($name, $crate::ctx_rtc_pack_check! {
                    binds = ($($rest)*)
                    binds_log_checking = ($($log_orig)*)
                    binds_log_original = ($($log_orig)*)
                })
            };
            ($name $name2) => {
                $crate::ctx_rtc_pack_check! {
                    binds = ($name $($rest)*)
                    binds_log_checking = ($($log)*)
                    binds_log_original = ($($log_orig)*)
                }
            };
        }
        __if_same_ident!($name $name2)
    }};
    {
        binds = ($name:ident $($rest:ident)*)
        binds_log_checking = (move $name2:ident $($log:tt)*)
        binds_log_original = ($($log_orig:tt)*)
    } => {{
        #[doc(hidden)]
        macro_rules! __if_same_ident {
            ($name $name) => {
                ($crate::macros::__Moved, $crate::ctx_rtc_pack_check! {
                    binds = ($($rest)*)
                    binds_log_checking = ($($log_orig)*)
                    binds_log_original = ($($log_orig)*)
                })
            };
            ($name $name2) => {
                $crate::ctx_rtc_pack_check! {
                    binds = ($name $($rest)*)
                    binds_log_checking = ($($log)*)
                    binds_log_original = ($($log_orig)*)
                }
            };
        }
        __if_same_ident!($name $name2)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! ctx_rtc_unpack {
    ($ctx:expr, $($binds:ident)*, $($log:tt)*) => {
        $crate::ctx_rtc_unpack_check! {
            dollar = ($)
            ctx = ($ctx)
            binds = ($($binds)*)
            binds_log_checking = ($($log)*)
            binds_log_original = ($($log)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! ctx_rtc_unpack_check {
    {
        dollar = ($d:tt)
        ctx = ($ctx:expr)
        binds = ()
        binds_log_checking = ($($log:tt)*)
        binds_log_original = ($($log_orig:tt)*)
    } => {};
    {
        dollar = ($d:tt)
        ctx = ($ctx:expr)
        binds = ($name:ident $($rest:ident)*)
        binds_log_checking = ()
        binds_log_original = ($($log_orig:tt)*)
     } => {
        ::core::compile_error!("")
    };
    {
        dollar = ($d:tt)
        ctx = ($ctx:expr)
        binds = ($name:ident $($rest:ident)*)
        binds_log_checking = (let $name2:ident $($log:tt)*)
        binds_log_original = ($($log_orig:tt)*)
    } => {
        #[doc(hidden)]
        macro_rules! __if {
            ($name == $name { $d ($d then:tt)* } else { $d ($d else:tt)* }) => { $d ($d then)* };
            ($name == $name2 { $d ($d then:tt)* } else { $d ($d else:tt)* }) => { $d ($d else)* };
        }
        __if!($name == $name2 {
            let $name = $ctx.0;
            $crate::ctx_rtc_unpack_check! {
                dollar = ($d)
                ctx = ($ctx.1)
                binds = ($($rest)*)
                binds_log_checking = ($($log_orig)*)
                binds_log_original = ($($log_orig)*)
            }
        } else {
            $crate::ctx_rtc_unpack_check! {
                dollar = ($d)
                ctx = ($ctx)
                binds = ($name $($rest)*)
                binds_log_checking = ($($log)*)
                binds_log_original = ($($log_orig)*)
            }
        })
    };
    {
        dollar = ($d:tt)
        ctx = ($ctx:expr)
        binds = ($name:ident $($rest:ident)*)
        binds_log_checking = (move $name2:ident $($log:tt)*)
        binds_log_original = ($($log_orig:tt)*)
    } => {
        #[doc(hidden)]
        macro_rules! __if {
            ($name == $name { $d ($d then:tt)* } else { $d ($d else:tt)* }) => { $d ($d then)* };
            ($name == $name2 { $d ($d then:tt)* } else { $d ($d else:tt)* }) => { $d ($d else)* };
        }
        __if!($name == $name2 {
            $crate::ctx_rtc_unpack_check! {
                dollar = ($d)
                ctx = ($ctx.1)
                binds = ($($rest)*)
                binds_log_checking = ($($log_orig)*)
                binds_log_original = ($($log_orig)*)
            }
        } else {
            $crate::ctx_rtc_unpack_check! {
                dollar = ($d)
                ctx = ($ctx)
                binds = ($name $($rest)*)
                binds_log_checking = ($($log)*)
                binds_log_original = ($($log_orig)*)
            }
        })
    };
}
