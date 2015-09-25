use std::marker::PhantomData;
use std::mem;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone)]
pub struct _0;
#[derive(Debug, Copy, Clone)]
pub struct _1;

/// Single type-level bit, `_0` or `_1`.
pub trait Bit { }
impl Bit for _0 { }
impl Bit for _1 { }

/// Nonnegative type-level integer, e.g., `((_1,_0),_1) = 0b101 = 25`.
pub trait Nat {
    fn reify() -> u64;
}
impl Nat for _0 { fn reify() -> u64 { 0 } }
impl Nat for _1 { fn reify() -> u64 { 1 } }
impl<N: Nat> Nat for (N, _0) {
    fn reify() -> u64 { N::reify() << 1 }
}
impl<N: Nat> Nat for (N, _1) {
    fn reify() -> u64 { (N::reify() << 1) | 1 }
}

/// Trait making GenericArray work
pub unsafe trait ArrayLength<T> : Nat {
	/// Associated type representing the array type for the number
	type ArrayType;
}

/// Empty array - needed to end recursion
#[allow(dead_code)]
pub struct EmptyArray<T> {
	_marker: PhantomData<T>
}

/// Array with a single element - for _1
#[allow(dead_code)]
pub struct UnitArray<T> {
	data: T
}

unsafe impl<T> ArrayLength<T> for _0 {
	type ArrayType = EmptyArray<T>;
}
unsafe impl<T> ArrayLength<T> for _1 {
	type ArrayType = UnitArray<T>;
}

#[allow(dead_code)]
pub struct GenericArrayImplEven<T, U> {
	parent1: U,
	parent2: U,
	_marker: PhantomData<T>
}

#[allow(dead_code)]
pub struct GenericArrayImplOdd<T, U> {
	parent1: U,
	parent2: U,
	data: T
}

unsafe impl<T, N: ArrayLength<T>> ArrayLength<T> for (N, _0) {
	type ArrayType = GenericArrayImplEven<T, N::ArrayType>;
}

unsafe impl<T, N: ArrayLength<T>> ArrayLength<T> for (N, _1) {
	type ArrayType = GenericArrayImplOdd<T, N::ArrayType>;
}

#[allow(dead_code)]
pub struct GenericArray<T, U: ArrayLength<T>> {
	data: U::ArrayType
}

impl<T, N> Index<usize> for GenericArray<T, N> where N: ArrayLength<T> {
	type Output = T;

	fn index(&self, i: usize) -> &T {
		assert!(i < N::reify() as usize);
		let p: *const T = self as *const GenericArray<T, N> as *const T;
		unsafe { &*p.offset(i as isize) }
	}
}

impl<T, N> IndexMut<usize> for GenericArray<T, N> where N: ArrayLength<T> {

	fn index_mut(&mut self, i: usize) -> &mut T {
		assert!(i < N::reify() as usize);
		let p: *mut T = self as *mut GenericArray<T, N> as *mut T;
		unsafe { &mut *p.offset(i as isize) }
	}
}

impl<T: Clone, N> GenericArray<T, N> where N: ArrayLength<T> {

	pub fn new() -> GenericArray<T, N> {
		unsafe { mem::zeroed() }
	}

	pub fn new_list(list: &[T]) -> GenericArray<T, N> {
		assert_eq!(list.len(), N::reify() as usize);
		let mut res = GenericArray::new();
		for i in 0..N::reify() as usize {
			res[i] = list[i].clone();
		}
		res
	}

}

#[cfg(test)]
mod test {
	use super::{_0, _1, GenericArray};

	type P97 = ((((((_1, _1), _0), _0), _0), _0), _1);

	#[test]
	fn test() {
		let mut list97 = [0; 97];
		for i in 0..97 {
			list97[i] = i as i32;
		}
	    let l : GenericArray<i32, P97> = GenericArray::new_list(&list97);
	    assert_eq!(l[0], 0);
	    assert_eq!(l[1], 1);
	    assert_eq!(l[32], 32);
	    assert_eq!(l[56], 56);
	}	
}
