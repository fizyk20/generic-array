use subtle::{ConditionallySelectable, ConstantTimeEq};

use crate::{ArrayLength, GenericArray};

impl<T, N: ArrayLength> ConstantTimeEq for GenericArray<T, N>
where
    T: ConstantTimeEq,
{
    #[inline]
    fn ct_eq(&self, other: &Self) -> subtle::Choice {
        self.as_slice().ct_eq(other.as_slice())
    }
}

impl<T, N: ArrayLength> ConditionallySelectable for GenericArray<T, N>
where
    GenericArray<T, N>: Copy,
    T: ConditionallySelectable,
{
    #[inline]
    fn conditional_select(a: &Self, b: &Self, choice: subtle::Choice) -> Self {
        let mut out = *a;
        out.conditional_assign(b, choice);
        out
    }

    #[inline]
    fn conditional_assign(&mut self, other: &Self, choice: subtle::Choice) {
        for (a, b) in self.iter_mut().zip(other.iter()) {
            a.conditional_assign(b, choice);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{arr, typenum::U4, GenericArray};
    use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

    #[test]
    fn ct_eq() {
        let a = arr![1u8, 2, 3, 4];
        let b = arr![1u8, 2, 3, 4];
        let c = arr![9u8, 2, 3, 4];

        assert!(bool::from(a.ct_eq(&b)));
        assert!(!bool::from(a.ct_eq(&c)));
    }

    #[test]
    fn conditional_select_and_assign() {
        let a = arr![1u8, 2, 3, 4];
        let c = arr![9u8, 8, 7, 6];

        let picked = GenericArray::<u8, U4>::conditional_select(&a, &c, Choice::from(1));
        assert_eq!(picked, c);
        let picked = GenericArray::<u8, U4>::conditional_select(&a, &c, Choice::from(0));
        assert_eq!(picked, a);

        let mut dst = a;
        dst.conditional_assign(&c, Choice::from(1));
        assert_eq!(dst, c);
        dst.conditional_assign(&a, Choice::from(0));
        assert_eq!(dst, c);
    }
}
