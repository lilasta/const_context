use const_context::action::Action;
use const_context::ctx;
use const_context::variable::Variable;

mod need_init {

    use super::*;

    static mut VALUE: Option<u32> = None;

    fn initialize_value() {
        unsafe { VALUE = Some(42) }
    }

    fn get_value() -> u32 {
        unsafe { VALUE.unwrap() }
    }

    #[derive(PartialEq, Eq)]
    pub struct Functions(());

    impl Functions {
        pub fn foo(&self) -> u32 {
            get_value()
        }
    }

    impl Variable for Functions {
        type Name = Self;
        type Type = Self;
    }

    pub fn initialize() -> impl Action<Output = ()> {
        ctx! {
            let _ = initialize_value();
            set Functions = Functions(());
        }
    }
}

fn main() {
    use need_init::Functions;

    // We cannot construct `Functions` ourself.
    //let functions = Functions(());

    let action = ctx! {
        need_init::initialize();
        funcs <- get Functions;
        let _ = println!("{}", funcs.foo());
    };

    action.run();
}
