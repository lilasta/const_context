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
    use core::any::TypeId;

    let a = TypeId::of::<A>();
    let b = TypeId::of::<B>();

    // SAFETY CHECKER.
    assert!(core::mem::size_of::<TypeId>() == core::mem::size_of::<u128>());

    // SAFETY: TypeId is just a wrapper for u128.
    unsafe { core::mem::transmute::<_, u128>(a) == core::mem::transmute::<_, u128>(b) }
}

/// Reinterprets the bits of a value of one type as another type.
/// This is the same as core::mem::transmute but it doesn't check whether the two types are of the same size.
pub(crate) const unsafe fn transmute<T, U>(from: T) -> U {
    use core::mem::ManuallyDrop;

    union Transmute<T, U> {
        from: ManuallyDrop<T>,
        to: ManuallyDrop<U>,
    }

    ManuallyDrop::into_inner(
        Transmute {
            from: ManuallyDrop::new(from),
        }
        .to,
    )
}

#[test]
#[cfg(test)]
fn test_of_is_same_type() {
    assert_eq!(true, is_same_type::<u64, u64>());
    assert_eq!(false, is_same_type::<u32, u64>());

    assert_eq!(true, { const { is_same_type::<u64, u64>() } });
    assert_eq!(false, { const { is_same_type::<u32, u64>() } });
}
