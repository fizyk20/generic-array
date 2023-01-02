//! Const default implementation

use crate::{ArrayLength, GenericArray, GenericArrayImplEven, GenericArrayImplOdd};
use const_default::ConstDefault;
use core::marker::PhantomData;

impl<T, U: ConstDefault> ConstDefault for GenericArrayImplEven<T, U> {
    const DEFAULT: Self = Self {
        parent1: U::DEFAULT,
        parent2: U::DEFAULT,
        _marker: PhantomData,
    };
}

impl<T: ConstDefault, U: ConstDefault> ConstDefault for GenericArrayImplOdd<T, U> {
    const DEFAULT: Self = Self {
        parent1: U::DEFAULT,
        parent2: U::DEFAULT,
        data: T::DEFAULT,
    };
}

impl<T, U: ArrayLength<T>> ConstDefault for GenericArray<T, U>
where
    U::ArrayType: ConstDefault,
{
    const DEFAULT: Self = Self {
        data: ConstDefault::DEFAULT,
    };
}
