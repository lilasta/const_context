#[test]
#[cfg(test)]
fn test() {
    use crate::action::Action;
    use crate::ctx;

    type Var = ((), u32);

    fn f(n: u32) -> impl Action<Output = u32> {
        ctx! {
            pure n
        }
    }

    let push90 = || {
        ctx! {
            set Var = 90;
        }
    };

    let action = ctx! {
        set Var = 45;
        set Var = a + b
        where
            a <- get Var,
            b <- get Var;
        get Var
    };

    let action2 = ctx! {
        v <- f(42);
        pure v
    };

    let action3 = ctx! {
        push90();
        v <- f(42);
        w <- get Var;
        pure (v + w)
    };

    assert_eq!(action.start_eval(), 90);
    assert_eq!(action2.start_eval(), 42);
    assert_eq!(action3.start_eval(), 132);

    let action = ctx! {
        f(42)
    };

    assert_eq!(action.start_eval(), 42);

    let action = ctx! {
        set Var = 90;
        get Var
    };

    let action2 = ctx! {
        action;
        get Var
    };

    assert_eq!(action2.start_eval(), 90);

    let action = ctx! {
        let _a = 0;
        let _a: u32 = 0;
        type Temp = (u64, u64);
        set Temp = 0;
        unset Temp;
    };

    assert_eq!(action.start_eval(), ());

    type Generic<T> = (T, u64);
    let action = ctx! {
        let _a = 0;
        let _a: u32 = 0;
        type Temp = (u64, u64);
        set Temp = 42;
        set Generic<T> = a + 0
        where
            const TEST: u32 = 0,
            T: 'static = u64,
            a <- get Generic<T>;
        unset Temp;
    };

    assert_eq!(action.start_eval(), ());

    const fn id_u64(n: u64) -> u64 {
        n
    }

    let action = ctx! {
        type Id = ((), (u64, ), u64);
        effect Id = id_u64;

        id_u64 <- effect Id;
        let a = id_u64(21);

        type Var = ((), u64);
        set Var = id_u64(21) where id_u64 <- effect Id;
        b <- get Var;

        pure (a + b)
    };

    assert_eq!(action.start_eval(), 42);
}
