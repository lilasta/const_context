use std::cell::UnsafeCell;
use std::rc::Rc;

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

    pub fn modify<'a>(&'a self, f: impl FnOnce(&mut T) + 'a) -> impl 'a + Action {
        ctx_if! {
            if set (Locked<ID>, ()) then
                ctx! { let _ = f(unsafe { &mut *UnsafeCell::raw_get(&self.0) }); }
            else
                ctx! { panic "Not locked" }
        }
    }
}

pub struct RcLock<const ID: Id, T>(Rc<UnsafeCell<T>>);

impl<const ID: Id, T> RcLock<ID, T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(UnsafeCell::new(value)))
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

    pub fn modify(&self, f: impl FnOnce(&mut T)) -> impl Action {
        let inner = self.0.clone();

        ctx_if! {
            if set (Locked<ID>, ()) then
                ctx! { let _ = f(unsafe { &mut *UnsafeCell::raw_get(&*inner) }); }
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

    let action = ctx! {
        let rclock = RcLock::<1, _>::new(1);
        rclock.lock();
        rclock.modify(|v| *v += 1);
        rclock.modify(|v| *v *= 21);
        rclock.modify(|v| println!("{}", *v));
        rclock.unlock();
    };
    action.start_eval();
}