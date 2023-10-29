use const_context::action::Action;
use const_context::ctx;

#[derive(PartialEq, Eq)]
enum Countdown {
    Three,
    Two,
    One,
    Go,
}

impl Countdown {
    const fn countdown(&self) -> Self {
        match self {
            Self::Three => Self::Two,
            Self::Two => Self::One,
            Self::One => Self::Go,
            Self::Go => panic!("Already started!"),
        }
    }
}

type Count = (Countdown, Countdown);

fn three() -> impl Action {
    ctx! {
        set Count = Countdown::Three;
        let _ = println!("three.");
    }
}

fn two() -> impl Action {
    ctx! {
        set Count = {
            assert!(matches!(count, Countdown::Three));
            count.countdown()
        } where count <- get Count;
        let _ = println!("two.");
    }
}

fn one() -> impl Action {
    ctx! {
        set Count = {
            assert!(matches!(count, Countdown::Two));
            count.countdown()
        } where count <- get Count;
        let _ = println!("one.");
    }
}

fn go() -> impl Action {
    ctx! {
        set Count = {
            assert!(matches!(count, Countdown::One));
            count.countdown()
        } where count <- get Count;
        let _ = println!("go!");
    }
}

fn main() {
    let action = ctx! {
        three();
        two();
        one();
        go();
    };

    action.run();
}
