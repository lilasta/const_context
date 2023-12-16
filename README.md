# const_context

> Note: This library relies on many nightly features. And this will never be production quality.

A Rust library for writing mixed code of constant expressions and runtime expressions with monadic syntax.

## License

CC0 1.0 Universal

## Cargo.toml

```toml
[dependencies]
const_context = { git = "https://github.com/lilasta/const_context" }
```

## `ctx!` macro

A macro for creating actions. All actions can be evaluated by calling `Action::run`.

```rust
use const_context::action::Action;

let action = ctx! {
    pure 42
};

assert_eq!(action.run(), 42);
```

The basic syntax is shown below:

|Syntax|Effect|
|-|-|
|`ACTION`|Just return `ACTION`.|
|`ACTION`; ...|Compose `ACTION` with subsequent actions.|
|`IDENT1` <- move `IDENT2`; ...|Move the value (`IDENT2`) from the runtime context and bind it to `IDENT1`. It can be used once.|
|`IDENT` <- `ACTION`; ...|Compose `ACTION` with subsequent actions. The result of `ACTION` can be used in subsequent actions with the name `IDENT`. "_" is also valid identifier.|
|let `IDENT` (: `TYPE`)? = `EXPRESSION`; ...|Bind `IDENT` to a value of `EXPRESSION` in the runtime context.|
|const `IDENT` : `TYPE` = `EXPRESSION`; ...|Declare constant value.|
|type `IDENT` = `TYPE`; ...|Declare type alias.|

The action syntax is shown below:

|Syntax|Action|
|-|-|
||Equivalent to `pure ()`.|
|pure `EXPRESSION`|Create an action that returns `EXPRESSION`.|
|get `VAR`|Get registered variable `VAR` in the const context. If `VAR` is not registered in the context, this will cause a compilation error.|
|set `VAR` = `EXPRESSION`|Register `EXPRESSION` as `VAR` in the const context. |
|unset `VAR`|Unregister `VAR` in the const context. If `VAR` is not registered in the context, this does nothing.|
|effect `EFFECT` = `FUNCTION`|Register `FUNCTION` as `EFFECT` in const contexts.|
|effect `EFFECT`|Get registered effect `EFFECT` in the const context. If `EFFECT` is not registered in the context, this will cause a compilation error.|
|panic "MESSAGE"|Panic when this action is instantiated. Any code that evaluates the action will cause an instantiation.|
|`EXPRESSION`|Create an action with `EXPRESSION`.|

`where` can be used in `set` action to use outer parameters:

```rust
fn where_set<const OUTER_VAL: usize, OuterT: 'static>() -> impl Action {
    ctx! {
        set VAR<T> = VAL + eff(val)
        where
            const VAL: usize = OUTER_VAL, // Using an outer const generic parameter
            val <- get SomeVar, // Using a variable in const contexts.
            eff <- effect SomeEffect, // Using an effect in const contexts.
            T: 'static = OuterT; // Using an outer generic type parameter.
        ...
    }
}
```

Other things to know:

- Continuation is not currently supported.
    - You can't use the effect feature to define `resume` or `return`.

## Registration of variables and effects

This library treats types as these names. In variables, it must implement `Variable` trait, in effects, it must implement `Effect` trait. But tuples implement them, so you don't have to write the implementation manually.

```rust
struct Name1;
struct Name2;

type Var = (Name1, usize); // This variable is named as `Name1` and has a value of type `usize`.
type Eff = (Name2, (usize,), bool); // This effect is named as `Name2`, takes a value of type `usize` as an argument and returns a value of type `bool`.

fn register() -> impl Action {
    const fn eff(n: usize) -> bool {
        n > 42
    }

    ctx! {
        set Var = 0usize;
        effect Eff = eff;
    }
}
```

Variables and effects are registered in const contexts, so it can also be used like this:

```rust
struct Name;

type Var = (Name, usize);

fn register_zero() -> impl Action {
    ctx! {
        set Var = 0usize;
    }
}

fn register_one() -> impl Action {
    ctx! {
        set Var = 1usize;
    }
}

fn action() -> impl Action<Output = usize> {
    ctx! {
        register_zero();
        zero <- get Var;

        register_one();
        one <- get Var;

        pure (zero + one)
    }
}
```