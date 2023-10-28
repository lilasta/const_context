use core::cell::UnsafeCell;

use const_context::action::Action;
use const_context::{ctx, ctx_if};

pub type Id = usize;

struct Locked<const ID: Id>;

pub struct Lock<const ID: Id, T>(UnsafeCell<T>);

impl<const ID: Id, T> Lock<ID, T> {
    pub const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    pub fn lock(&self) -> impl Action {
        ctx_if! {
            if set (Locked<ID>, ()) then
                ctx! { panic "Double locks" }
            else
                ctx! { set Locked<ID>: () = () where const ID: Id = ID; }
        }
    }

    pub fn unlock(&self) -> impl Action {
        ctx! {
            unset (Locked<ID>, ());
        }
    }

    pub fn modify<'a>(&'a self, f: impl FnOnce(&'a mut T) + 'a) -> impl 'a + Action {
        ctx_if! {
            if set (Locked<ID>, ()) then
                ctx! { let _ = f(unsafe { &mut *UnsafeCell::raw_get(&self.0) }); }
            else
                ctx! { panic "Not locked" }
        }
    }
}

fn main() {
    let ref lock = Lock::<1, _>::new(1);

    let action = ctx! {
        lock.lock();
        lock.modify(|v| *v += 1);
        lock.modify(|v| *v *= 21);
        lock.modify(|v| println!("{}", *v));
        lock.unlock();
    };

    action.start_eval();
}
