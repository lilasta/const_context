use const_context::action::Action;
use const_context::ctx;

struct Id<const N: usize>;
type Var1 = (Id<1>, u32);
type Var2 = (Id<2>, u32);
type Var3 = (Id<3>, u32);

fn main() {
    let action = ctx! {
        set Var1 = 42;
        v1 <- get Var1;
        set Var1 = 8;
        v2 <- get Var1;
        set Var1 = v + 42 where v <- get Var1;
        v3 <- get Var1;
        pure (v1 + v2 + v3)
    };

    println!("{}", action.run()); // 100

    let action = ctx! {
        set Var1 = 6;
        set Var2 = 7;
        set Var3 = a * b where a <- get Var1, b <- get Var2;
        get Var3
    };

    println!("{}", action.run()); // 42
}
