pub trait ConstValue {
    type Type;
    const VALUE: Self::Type;
}

#[doc(hidden)]
pub const unsafe fn __transmute_copy_hack<T, U>(src: T) -> U {
    let ret = core::mem::transmute_copy(&src);
    core::mem::forget(src);
    ret
}
