use std::marker::PhantomData;
use std::mem;
use std::ops::{Index, IndexMut};

// Code from dimensioned - START

pub struct Zero;

/// For any non-negative Peano number `N`, we define its successor, `Succ<N>`.
///
/// This gives us positive Peano numbers.
#[derive(Copy, Clone)]
pub struct Succ<N: Peano> {
    _marker: PhantomData<N>
}


/// The Peano number +1; `P1 = Succ<Zero>;`
pub type P1 = Succ<Zero>;
/// The Peano number +2; `P2 = Succ<P1>;`
pub type P2 = Succ<P1>;
/// The Peano number +3; `P3 = Succ<P2>;`
pub type P3 = Succ<P2>;
/// The Peano number +4; `P4 = Succ<P3>;`
pub type P4 = Succ<P3>;
/// The Peano number +5; `P5 = Succ<P4>;`
pub type P5 = Succ<P4>;
/// The Peano number +6; `P6 = Succ<P5>;`
pub type P6 = Succ<P5>;
/// The Peano number +7; `P7 = Succ<P6>;`
pub type P7 = Succ<P6>;
/// The Peano number +8; `P8 = Succ<P7>;`
pub type P8 = Succ<P7>;
/// The Peano number +9; `P9 = Succ<P8>;`
pub type P9 = Succ<P8>;

pub trait Peano {}
impl Peano for Zero {}
impl<N: Peano> Peano for Succ<N> {}

trait PrevPeano: Peano {
	type Prev: Peano;
}

impl<N: Peano> PrevPeano for Succ<N> {
	type Prev = N;
}

pub trait ToInt: Peano {
    #[allow(missing_docs)]
    fn to_int() -> i32;
}
impl ToInt for Zero {
    fn to_int() -> i32 { 0 }
}
impl<N: Peano + ToInt> ToInt for Succ<N> {
    fn to_int() -> i32 { 1 + N::to_int() }
}

// Code from dimensioned - END

/// Empty array - needed to end recursion
#[allow(dead_code)]
struct EmptyArray<T> {
	_marker: PhantomData<T>
}

/// Type implementing the array recursion
#[allow(dead_code)]
#[repr(packed)]
struct GenericArrayImpl<T, U: ArrayLength<T>> {
	prev: U::ArrayPrev,
	val: T
}

/// The actually useful array type
#[allow(dead_code)]
#[repr(packed)]
struct GenericArray<T, U: ArrayLength<T>> {
	data: U::ArrayPrev
}

/// Trait making GenericArray work
trait ArrayLength<T> : Peano + ToInt {
	/// Associated type representing the array type for the number that is one less
	type ArrayPrev;
}

// Array of size Zero should be empty
impl<T> ArrayLength<T> for Zero {
	type ArrayPrev = EmptyArray<T>;
}
// We use GenericArrayImpl for each next number
impl<T, N> ArrayLength<T> for Succ<N> where N: ArrayLength<T> {
	type ArrayPrev = GenericArrayImpl<T, N>;
}

impl<T, N> Index<usize> for GenericArray<T, N> where N: ArrayLength<T> {
	type Output = T;

	fn index(&self, i: usize) -> &T {
		assert!(i < N::to_int() as usize);
		let p: *const T = self as *const GenericArray<T, N> as *const T;
		unsafe { &*p.offset(i as isize) }
	}
}

impl<T, N> IndexMut<usize> for GenericArray<T, N> where N: ArrayLength<T> {

	fn index_mut(&mut self, i: usize) -> &mut T {
		assert!(i < N::to_int() as usize);
		let p: *mut T = self as *mut GenericArray<T, N> as *mut T;
		unsafe { &mut *p.offset(i as isize) }
	}
}

impl<T: Clone, N> GenericArray<T, N> where N: ArrayLength<T> {

	fn new() -> GenericArray<T, N> {
		unsafe { mem::zeroed() }
	}

	fn new_list(list: &[T]) -> GenericArray<T, N> {
		assert_eq!(list.len(), N::to_int() as usize);
		let mut res = GenericArray::new();
		for i in 0..N::to_int() as usize {
			res[i] = list[i].clone();
		}
		res
	}

}

fn main() {
    let l : GenericArray<i32, P4> = GenericArray::new_list(&[1, 2, 3, 4]);
    println!("l[0]: {}", l[0]);
    println!("l[1]: {}", l[1]);
    println!("l[2]: {}", l[2]);
    println!("l[3]: {}", l[3]);
}