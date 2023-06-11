use const_context::action::Action;
use const_context::ctx;

type ForEachHandler<T> = ((), (T,), ());

fn for_each<T: 'static, const N: usize>(l: [T; N]) -> impl Action {
    ctx! {
       handler <- effect ForEachHandler<T>;
       let _ = l.into_iter().for_each(handler);
    }
}

fn handler_i32(n: i32) {
    println!("{}", n);
}

fn main() {
    let test = [0, 1, 2, 3, 4];
    let action = ctx! {
        effect ForEachHandler<i32> = handler_i32;
        for_each(test);
    };

    action.start_eval();
}
