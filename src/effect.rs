use core::marker::{PhantomData, Tuple};
use core::mem::MaybeUninit;

use crate::utils::type_eq;

pub trait Effect {
    type Name: 'static;
    type Args: 'static + Tuple;
    type Ret: 'static;
}

impl<Name, Args, Ret> Effect for (Name, Args, Ret)
where
    Name: 'static,
    Args: 'static + Tuple,
    Ret: 'static,
{
    type Name = Name;
    type Args = Args;
    type Ret = Ret;
}

pub struct EffectListEnd;

pub struct EffectListHas<Name, Args, Effect, Next>(PhantomData<(Name, Args, Effect, Next)>);

pub trait EffectList {
    type Name: 'static;
    type Next: EffectList;
    type Args: 'static + Tuple;
    type Effect: 'static + Fn<Self::Args>;
    const IS_END: bool;
}

impl EffectList for EffectListEnd {
    type Name = ();
    type Next = EffectListEnd;
    type Args = ();
    type Effect = fn();
    const IS_END: bool = true;
}

impl<Name: 'static, Args: 'static + Tuple, Effect: 'static + Fn<Args>, Next: EffectList> EffectList
    for EffectListHas<Name, Args, Effect, Next>
{
    type Name = Name;
    type Next = Next;
    type Args = Args;
    type Effect = Effect;
    const IS_END: bool = false;
}

#[track_caller]
pub fn call<List, Name, Args, Ret>(args: Args) -> Ret
where
    List: EffectList,
    Name: 'static,
    Args: 'static + Tuple,
{
    assert!(!List::IS_END);

    if type_eq::<Name, List::Name>() && type_eq::<Args, List::Args>() {
        unsafe {
            let f = MaybeUninit::<List::Effect>::uninit().assume_init();
            let ret = core::mem::transmute_copy::<_, Ret>(
                &f.call(core::mem::transmute_copy::<_, List::Args>(&args)),
            );
            return ret;
        };
    }

    call::<List::Next, Name, Args, Ret>(args)
}
