pub trait ConstValue {
    type Type;
    const VALUE: Self::Type;
}
