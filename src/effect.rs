use core::marker::{Destruct, PhantomData, Tuple};
use core::mem::MaybeUninit;

use crate::utils::type_eq;

pub trait Effect {
    type Name: 'static;
    type Args: 'static + Tuple + ~const Destruct;
    type Output: 'static + ~const Destruct;
}

impl Effect for () {
    type Name = ();
    type Args = ();
    type Output = ();
}

impl<Name, Args, Output> Effect for (Name, Args, Output)
where
    Name: 'static,
    Args: 'static + Tuple,
    Output: 'static + ~const Destruct,
{
    type Name = Name;
    type Args = Args;
    type Output = Output;
}

pub struct EffectListEnd;

pub struct EffectListHas<Eff, EffConcrete, Next>(PhantomData<(Eff, EffConcrete, Next)>);

pub trait EffectList {
    type Effect: Effect;
    type EffectConcrete: 'static
        + ~const Fn<<Self::Effect as Effect>::Args, Output = <Self::Effect as Effect>::Output>;

    type Next: EffectList;
    const IS_END: bool;
}

impl EffectList for EffectListEnd {
    type Effect = ();
    type EffectConcrete = fn();

    type Next = EffectListEnd;
    const IS_END: bool = true;
}

impl<Eff, EffConcrete, Next: EffectList> EffectList for EffectListHas<Eff, EffConcrete, Next>
where
    Eff: Effect,
    EffConcrete: 'static + Fn<Eff::Args, Output = Eff::Output>,
{
    type Effect = Eff;
    type EffectConcrete = EffConcrete;

    type Next = Next;
    const IS_END: bool = false;
}

#[track_caller]
pub const fn call<List, Eff>(args: Eff::Args) -> Eff::Output
where
    List: EffectList,
    Eff: Effect,
{
    assert!(!List::IS_END);

    if const {
        type_eq::<Eff::Name, <List::Effect as Effect>::Name>()
            && type_eq::<Eff::Args, <List::Effect as Effect>::Args>()
            && type_eq::<Eff::Output, <List::Effect as Effect>::Output>()
    } {
        unsafe {
            #[allow(invalid_value)]
            let func = MaybeUninit::<&'static List::EffectConcrete>::uninit().assume_init();
            let args = core::mem::transmute_copy::<_, <List::Effect as Effect>::Args>(&args);
            let ret = func.call(args);
            return core::mem::transmute_copy::<_, Eff::Output>(&ret);
        };
    }

    call::<List::Next, Eff>(args)
}

#[track_caller]
pub const fn get<List, Eff>() -> EffectWrapper<Eff>
where
    List: EffectList,
    Eff: Effect,
{
    EffectWrapper(call::<List, Eff>)
}

pub struct EffectWrapper<Eff: Effect>(fn(Eff::Args) -> Eff::Output);

impl<Eff: Effect> const FnOnce<Eff::Args> for EffectWrapper<Eff>
where
    fn(Eff::Args) -> Eff::Output: ~const FnOnce(Eff::Args) -> Eff::Output,
{
    type Output = Eff::Output;

    extern "rust-call" fn call_once(self, args: Eff::Args) -> Self::Output {
        self.0.call_once((args,))
    }
}

impl<Eff: Effect> const FnMut<Eff::Args> for EffectWrapper<Eff>
where
    fn(Eff::Args) -> Eff::Output: ~const FnMut(Eff::Args) -> Eff::Output,
{
    extern "rust-call" fn call_mut(&mut self, args: Eff::Args) -> Self::Output {
        self.0.call_mut((args,))
    }
}

impl<Eff: Effect> const Fn<Eff::Args> for EffectWrapper<Eff>
where
    fn(Eff::Args) -> Eff::Output: ~const Fn(Eff::Args) -> Eff::Output,
{
    extern "rust-call" fn call(&self, args: Eff::Args) -> Self::Output {
        self.0.call((args,))
    }
}
