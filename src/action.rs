pub mod bind;
pub mod effect_get;
pub mod effect_set;
pub mod r#if;
pub mod lazy;
pub mod pure;
pub mod variable_get;
pub mod variable_isset;
pub mod variable_set;
pub mod variable_unset;

use crate::context::{ConstContext, InitialActionContext};

pub trait Action: Sized {
    type Output;
    type Context<Ctx: ConstContext>: ConstContext;

    #[inline(always)]
    fn run(self) -> Self::Output {
        self.run_with::<InitialActionContext>()
    }

    fn run_with<Ctx: ConstContext>(self) -> Self::Output;
}
