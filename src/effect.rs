use core::marker::{Destruct, PhantomData, Tuple};
//use core::mem::MaybeUninit;

use crate::utils::is_same_type;

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

pub struct EffectListHas<Eff, EffConcrete, Rest>(PhantomData<(Eff, EffConcrete, Rest)>);

pub trait EffectList {
    type Effect: Effect;
    // TODO: Effect is not available until `Fn` is marked as `#[const_trait]` again
    //type EffectConcrete: ~const Fn<<Self::Effect as Effect>::Args, Output = <Self::Effect as Effect>::Output>
    //    + ~const Destruct;
    type EffectConcrete: ~const Destruct;

    type Rest: EffectList;
    const IS_END: bool;
}

impl EffectList for EffectListEnd {
    type Effect = ();
    type EffectConcrete = fn();

    type Rest = EffectListEnd;
    const IS_END: bool = true;
}

impl<Eff, EffConcrete, Rest: EffectList> EffectList for EffectListHas<Eff, EffConcrete, Rest>
where
    Eff: Effect,
    EffConcrete: 'static + Fn<Eff::Args, Output = Eff::Output>,
{
    type Effect = Eff;
    type EffectConcrete = EffConcrete;

    type Rest = Rest;
    const IS_END: bool = false;
}

#[inline(always)]
pub const fn call<List, Eff>(args: Eff::Args) -> Eff::Output
where
    List: EffectList,
    Eff: Effect,
{
    assert!(!List::IS_END);

    if const {
        is_same_type::<Eff::Name, <List::Effect as Effect>::Name>()
            && is_same_type::<Eff::Args, <List::Effect as Effect>::Args>()
            && is_same_type::<Eff::Output, <List::Effect as Effect>::Output>()
    } {
        // TODO: Effect is not available until `Fn` is marked as `#[const_trait]` again
        /*
        unsafe {
            #[allow(invalid_value)]
            let func = MaybeUninit::<List::EffectConcrete>::uninit().assume_init();
            let args_copied = core::mem::transmute_copy::<_, <List::Effect as Effect>::Args>(&args);

            let ret = func.call(args_copied);
            let ret_copied = core::mem::transmute_copy::<_, Eff::Output>(&ret);

            core::mem::forget(func);
            core::mem::forget(args);
            core::mem::forget(ret);

            return ret_copied;
        };
        */
        panic!("Effect is not available until `Fn` is marked as `#[const_trait]` again")
    } else {
        call::<List::Rest, Eff>(args)
    }
}

#[inline(always)]
pub const fn get<List, Eff>() -> RuntimeEffect<Eff>
where
    List: EffectList,
    Eff: Effect,
{
    RuntimeEffect(call::<List, Eff>)
}

#[inline(always)]
pub const fn get_const<List, Eff>() -> ConstEffect<List, Eff>
where
    List: EffectList,
    Eff: Effect,
{
    ConstEffect(PhantomData)
}

pub struct RuntimeEffect<Eff: Effect>(fn(Eff::Args) -> Eff::Output);

impl<Eff: Effect> FnOnce<Eff::Args> for RuntimeEffect<Eff> {
    type Output = Eff::Output;

    #[inline(always)]
    extern "rust-call" fn call_once(self, args: Eff::Args) -> Self::Output {
        self.0.call_once((args,))
    }
}

impl<Eff: Effect> FnMut<Eff::Args> for RuntimeEffect<Eff> {
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, args: Eff::Args) -> Self::Output {
        self.0.call_mut((args,))
    }
}

impl<Eff: Effect> Fn<Eff::Args> for RuntimeEffect<Eff> {
    #[inline(always)]
    extern "rust-call" fn call(&self, args: Eff::Args) -> Self::Output {
        self.0.call((args,))
    }
}

pub struct ConstEffect<List: EffectList, Eff: Effect>(PhantomData<(List, Eff)>);

impl<List: EffectList, Eff: Effect> const FnOnce<Eff::Args> for ConstEffect<List, Eff> {
    type Output = Eff::Output;

    #[inline(always)]
    extern "rust-call" fn call_once(self, args: Eff::Args) -> Self::Output {
        call::<List, Eff>(args)
    }
}

impl<List: EffectList, Eff: Effect> const FnMut<Eff::Args> for ConstEffect<List, Eff> {
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, args: Eff::Args) -> Self::Output {
        call::<List, Eff>(args)
    }
}

impl<List: EffectList, Eff: Effect> const Fn<Eff::Args> for ConstEffect<List, Eff> {
    #[inline(always)]
    extern "rust-call" fn call(&self, args: Eff::Args) -> Self::Output {
        call::<List, Eff>(args)
    }
}
