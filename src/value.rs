use core::marker::ConstParamTy;

#[derive(Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub struct ConstValue(&'static [u8]);

impl ConstValue {
    pub const fn new<T>(value: T) -> Self {
        let size = core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>();

        let bytes = unsafe {
            let ptr = core::intrinsics::const_allocate(size, align);
            core::ptr::write(ptr.cast(), value);
            core::slice::from_raw_parts(ptr.cast(), size)
        };

        Self(bytes)
    }

    pub const fn with_type<T>(self) -> T {
        let Self(bytes) = self;
        unsafe { core::ptr::read(bytes.as_ptr().cast()) }
    }
}