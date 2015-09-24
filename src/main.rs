use std::marker::PhantomData;
use std::mem;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone)]
struct _0;
#[derive(Debug, Copy, Clone)]
struct _1;

/// Single type-level bit, `_0` or `_1`.
trait Bit { }
impl Bit for _0 { }
impl Bit for _1 { }

/// Nonnegative type-level integer, e.g., `((_1,_0),_1) = 0b101 = 25`.
trait Nat {
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

/// Positive type-level integer.
trait Pos: Nat { }
impl Pos for _1 { }
impl<N: Pos> Pos for (N, _0) { }
impl<N: Nat> Pos for (N, _1) { }

/// Trait making GenericArray work
trait ArrayLength<T> : Nat {
	/// Associated type representing the array type for the number
	type ArrayType;
}

/// Empty array - needed to end recursion
#[allow(dead_code)]
struct EmptyArray<T> {
	_marker: PhantomData<T>
}

/// Array with a single element - for _1
#[allow(dead_code)]
struct UnitArray<T> {
	data: T
}

impl<T> ArrayLength<T> for _0 {
	type ArrayType = EmptyArray<T>;
}
impl<T> ArrayLength<T> for _1 {
	type ArrayType = UnitArray<T>;
}

#[allow(dead_code)]
struct GenericArrayImplEven<T, U> {
	parent1: U,
	parent2: U,
	_marker: PhantomData<T>
}

#[allow(dead_code)]
struct GenericArrayImplOdd<T, U> {
	parent1: U,
	parent2: U,
	data: T
}

impl<T, N: ArrayLength<T>> ArrayLength<T> for (N, _0) {
	type ArrayType = GenericArrayImplEven<T, N::ArrayType>;
}

impl<T, N: ArrayLength<T>> ArrayLength<T> for (N, _1) {
	type ArrayType = GenericArrayImplOdd<T, N::ArrayType>;
}

#[allow(dead_code)]
struct GenericArray<T, U: ArrayLength<T>> {
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

	fn new() -> GenericArray<T, N> {
		unsafe { mem::zeroed() }
	}

	fn new_list(list: &[T]) -> GenericArray<T, N> {
		assert_eq!(list.len(), N::reify() as usize);
		let mut res = GenericArray::new();
		for i in 0..N::reify() as usize {
			res[i] = list[i].clone();
		}
		res
	}

}

type P65 = ((((((_1, _0), _0), _0), _0), _0), _1);

fn main() {
    let l : GenericArray<i32, P65> = GenericArray::new_list(&[1; 65]);
    println!("l[0]: {}", l[0]);
    println!("l[1]: {}", l[1]);
    println!("l[2]: {}", l[2]);
    println!("l[3]: {}", l[3]);
}