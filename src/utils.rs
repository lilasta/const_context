use core::any::TypeId;
use core::intrinsics::const_allocate;

/// Check whether type A and type B are the same.
/// ```rust,ignore
/// assert_eq!(true, is_same_type::<u64, u64>());
/// assert_eq!(false, is_same_type::<u32, u64>());
/// ```
pub(crate) const fn is_same_type<A, B>() -> bool
where
    A: ?Sized + 'static,
    B: ?Sized + 'static,
{
    let a = TypeId::of::<A>();
    let b = TypeId::of::<B>();

    // SAFETY CHECKER.
    assert!(core::mem::size_of::<TypeId>() == core::mem::size_of::<u128>());

    // SAFETY: TypeId is just a wrapper for u128.
    unsafe { core::mem::transmute::<_, u128>(a) == core::mem::transmute::<_, u128>(b) }
}

/// Concatenate two strings. This can only be used at compile-time.
/// ```rust,ignore
/// const STR1: &'static str = "str_concat";
/// const STR2: &'static str = "_ct";
/// const CONCATENATED: &'static str = str_concat_ct(STR1, STR2); // "str_concat_ct"
/// ```
pub(crate) const fn str_concat_ct(s1: &str, s2: &str) -> &'static str {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let len = s1.len() + s2.len();

    // SAFETY: If there is an error, it should only be a compilation error.
    // SAFETY: The correctness of the values is checked by testing.
    unsafe {
        let ptr = const_allocate(
            core::mem::size_of::<u8>() * len,
            core::mem::align_of::<u8>(),
        );
        core::ptr::copy(s1.as_ptr(), ptr, s1.len());
        core::ptr::copy(s2.as_ptr(), ptr.add(s1.len()), s2.len());
        core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr.cast(), len))
    }
}

#[test]
#[cfg(test)]
fn test_of_is_same_type() {
    assert_eq!(true, is_same_type::<u64, u64>());
    assert_eq!(false, is_same_type::<u32, u64>());

    assert_eq!(true, { const { is_same_type::<u64, u64>() } });
    assert_eq!(false, { const { is_same_type::<u32, u64>() } });
}

#[test]
#[cfg(test)]
fn test_of_str_concat_ct() {
    assert_eq!("test", { const { str_concat_ct("test", "") } });
    assert_eq!("test", { const { str_concat_ct("", "test") } });
    assert_eq!("test", { const { str_concat_ct("te", "st") } });
    assert_eq!("😊😊", { const { str_concat_ct("😊", "😊") } });
}
