//! Provides compile-time and runtime algorithms for safely generating `GenericArray` instances.
//!
//! These all use wrapper structures internally for generating and consuming arrays in a way that
//! properly drops all created or remaining valid elements when the parent structure is dropped,
//! such as when the mapping functions panic.

use super::{ArrayLength, GenericArray};
use core::{mem, ptr};
use core::marker::PhantomData;
use core::ops::{Add, Sub};

use nodrop::NoDrop;
use typenum::{Add1, IsLess, Same, Sub1, Unsigned};
use typenum::bit::{B1, Bit};
use typenum::consts::{True, U0};
use typenum::uint::{UInt, UTerm};

struct ArrayConsumer<T, N: ArrayLength<T>> {
    array: NoDrop<GenericArray<T, N>>,
    position: usize,
}

impl<T, N: ArrayLength<T>> ArrayConsumer<T, N> {
    fn new(array: GenericArray<T, N>) -> ArrayConsumer<T, N> {
        ArrayConsumer {
            array: NoDrop::new(array),
            position: 0,
        }
    }
}

impl<T, N: ArrayLength<T>> Drop for ArrayConsumer<T, N> {
    fn drop(&mut self) {
        for i in self.position..N::to_usize() {
            unsafe {
                ptr::drop_in_place(self.array.get_unchecked_mut(i));
            }
        }
    }
}

struct ArrayBuilder<T, N: ArrayLength<T>> {
    array: NoDrop<GenericArray<T, N>>,
    position: usize,
}

impl<T, N: ArrayLength<T>> ArrayBuilder<T, N> {
    fn new() -> ArrayBuilder<T, N> {
        ArrayBuilder {
            array: NoDrop::new(unsafe { mem::uninitialized() }),
            position: 0,
        }
    }
}

impl<T, N: ArrayLength<T>> Drop for ArrayBuilder<T, N> {
    fn drop(&mut self) {
        for value in self.array.iter_mut().take(self.position) {
            unsafe {
                ptr::drop_in_place(value);
            }
        }
    }
}

/// Initializes a new `GenericArray` instance using the given function.
///
/// If the generator function panics while initializing the array,
/// any already initialized elements will be dropped.
#[inline]
pub fn generate<T, N, F>(f: F) -> GenericArray<T, N>
where
    F: Fn(usize) -> T,
    N: ArrayLength<T>,
{
    let mut destination = ArrayBuilder::new();

    for (i, dst) in destination.array.iter_mut().enumerate() {
        unsafe {
            ptr::write(dst, f(i));
        }

        destination.position += 1;
    }

    let array = unsafe { ptr::read(&destination.array) };

    mem::forget(destination);

    array.into_inner()
}

/// Maps a `GenericArray` to another `GenericArray`.
///
/// If the mapping function panics, any already initialized elements in the new array
/// will be dropped, AND any unused elements in the source array will also be dropped.
#[inline]
pub fn map<T, U, N, F>(array: GenericArray<T, N>, f: F) -> GenericArray<U, N>
where
    F: Fn(T) -> U,
    N: ArrayLength<T> + ArrayLength<U>,
{
    let mut source = ArrayConsumer::new(array);
    let mut destination = ArrayBuilder::new();

    for (dst, src) in destination.array.iter_mut().zip(source.array.iter()) {
        unsafe {
            ptr::write(dst, f(ptr::read(src)));
        }

        source.position += 1;
        destination.position += 1;
    }

    let array = unsafe { ptr::read(&destination.array) };

    mem::forget((source, destination));

    array.into_inner()
}

/// Combines two `GenericArray` instances and iterates through both of them,
/// initializing a new `GenericArray` with the result of the zipped mapping function.
///
/// If the mapping function panics, any already initialized elements in the new array
/// will be dropped, AND any unused elements in the source arrays will also be dropped.
#[inline]
pub fn zip<A, B, T, N, F>(
    lhs: GenericArray<A, N>,
    rhs: GenericArray<B, N>,
    f: F,
) -> GenericArray<T, N>
where
    F: Fn(A, B) -> T,
    N: ArrayLength<A> + ArrayLength<B> + ArrayLength<T>,
{
    let mut left = ArrayConsumer::new(lhs);
    let mut right = ArrayConsumer::new(rhs);

    let mut destination = ArrayBuilder::new();

    for (dst, (lhs, rhs)) in
        destination.array.iter_mut().zip(left.array.iter().zip(
            right.array.iter(),
        ))
    {
        unsafe {
            ptr::write(dst, f(ptr::read(lhs), ptr::read(rhs)));
        }

        destination.position += 1;
        left.position += 1;
        right.position += 1;
    }

    let array = unsafe { ptr::read(&destination.array) };

    mem::forget((left, right, destination));

    array.into_inner()
}

/// Provides compile-time and pseudo-runtime algorithms for generating `GenericArray` instances.
///
/// Initialization methods for `RecursiveArrayBuilder` are designed so that if
/// an error occurs while generating the array, any already initialized elements will be dropped.
///
/// Most of these use type-level recursion to iterate `GenericArray`s,
/// so most expressions can be inlined and simplified at compile-time.
///
/// The `IterableArrayLength` trait is implemented for basically any
pub struct RecursiveArrayBuilder<T, N: ArrayLength<T>, P: Unsigned> {
    array: NoDrop<GenericArray<T, N>>,
    _position: PhantomData<P>,
}

impl<T, N: ArrayLength<T>> RecursiveArrayBuilder<T, N, U0> {
    /// Creates a new `RecursiveArrayBuilder` that starts at the first element.
    #[must_use]
    pub fn new() -> RecursiveArrayBuilder<T, N, U0> {
        RecursiveArrayBuilder {
            array: NoDrop::new(unsafe { mem::uninitialized() }),
            _position: PhantomData,
        }
    }

    /// Initializes a new `GenericArray` instance using the given function.
    ///
    /// If the generator function panics while initializing the array,
    /// any already initialized elements will be dropped.
    #[inline(always)]
    pub fn generate<F>(generator: F) -> GenericArray<T, N>
    where
        F: Fn(usize) -> T,
        N: IterableArrayLength<T, N>,
    {
        <N as IterableArrayLength<T, N>>::generate(
            RecursiveArrayBuilder {
                array: NoDrop::new(unsafe { mem::uninitialized() }),
                _position: PhantomData,
            },
            generator,
        )
    }

    /// Maps a `GenericArray` to another `GenericArray`.
    ///
    /// If the mapping function panics, any already initialized elements in the new array
    /// will be dropped, AND any unused elements in the source array will also be dropped.
    pub fn map<U, F>(source: GenericArray<U, N>, f: F) -> GenericArray<T, N>
    where
        F: Fn(U) -> T,
        N: ArrayLength<U> + IterableArrayLength<T, N>,
    {
        <N as IterableArrayLength<T, N>>::map(
            RecursiveArrayBuilder {
                array: NoDrop::new(unsafe { mem::uninitialized() }),
                _position: PhantomData,
            },
            GenericArrayConsumer::new(source),
            f,
        )
    }

    /// Maps a `GenericArray` to another `GenericArray` by reference.
    ///
    /// If the mapping function panics, any already initialized elements will be dropped.
    pub fn map_ref<U, F>(source: &GenericArray<U, N>, f: F) -> GenericArray<T, N>
    where
        F: Fn(&U) -> T,
        N: ArrayLength<U> + IterableArrayLength<T, N>,
    {
        RecursiveArrayBuilder::generate(move |i| f(unsafe { source.get_unchecked(i) }))
    }

    /// Combines two `GenericArray` instances and iterates through both of them,
    /// initializing a new `GenericArray` with the result of the zipped mapping function.
    ///
    /// If the mapping function panics, any already initialized elements in the new array
    /// will be dropped, AND any unused elements in the source arrays will also be dropped.
    pub fn zip<A, B, F>(
        lhs: GenericArray<A, N>,
        rhs: GenericArray<B, N>,
        f: F,
    ) -> GenericArray<T, N>
    where
        F: Fn(A, B) -> T,
        N: ArrayLength<A> + ArrayLength<B> + IterableArrayLength<T, N>,
    {
        <N as IterableArrayLength<T, N>>::zip(
            RecursiveArrayBuilder {
                array: NoDrop::new(unsafe { mem::uninitialized() }),
                _position: PhantomData,
            },
            GenericArrayConsumer::new(lhs),
            GenericArrayConsumer::new(rhs),
            f,
        )
    }

    /// Combines two `GenericArray` instances and iterates through both of them by reference,
    /// initializing a new `GenericArray` with the result of the zipped mapping function.
    ///
    /// If the mapping function panics, any already initialized elements will be dropped.
    pub fn zip_ref<A, B, F>(
        lhs: &GenericArray<A, N>,
        rhs: &GenericArray<B, N>,
        f: F,
    ) -> GenericArray<T, N>
    where
        F: Fn(&A, &B) -> T,
        N: ArrayLength<A> + ArrayLength<B> + IterableArrayLength<T, N>,
    {
        RecursiveArrayBuilder::generate(move |i| unsafe {
            f(lhs.get_unchecked(i), rhs.get_unchecked(i))
        })
    }
}

impl<T, N: ArrayLength<T>, P: Unsigned> RecursiveArrayBuilder<T, N, P>
where
    P: IsLess<N, Output = True>,
    P: Add<B1>,
    Add1<P>: Unsigned,
{
    /// Apply the generator function to the current element and
    /// return a builder for the next element.
    #[must_use]
    pub fn next<F>(self, f: F) -> RecursiveArrayBuilder<T, N, Add1<P>>
    where
        F: Fn() -> T,
    {
        RecursiveArrayBuilder {
            array: unsafe {
                // move array out of self
                let mut arr = ptr::read(&self.array);

                // forget self
                mem::forget(self);

                // write new value to position
                ptr::write(arr.get_unchecked_mut(P::to_usize()), f());

                arr
            },
            _position: PhantomData,
        }
    }
}

impl<T, N: ArrayLength<T>, P: Unsigned> RecursiveArrayBuilder<T, N, P>
where
    P: Same<N>,
{
    /// Returns the finished `GenericArray` instance.
    pub fn finish(self) -> GenericArray<T, N> {
        let arr = unsafe { ptr::read(&self.array) };

        mem::forget(self);

        arr.into_inner()
    }
}

impl<T, N: ArrayLength<T>, P: Unsigned> Drop for RecursiveArrayBuilder<T, N, P> {
    fn drop(&mut self) {
        for value in self.array.iter_mut().take(P::to_usize()) {
            unsafe {
                ptr::drop_in_place(value);
            }
        }
    }
}

#[doc(hidden)]
pub struct GenericArrayConsumer<T, N: ArrayLength<T>, P: Unsigned> {
    array: NoDrop<GenericArray<T, N>>,
    _position: PhantomData<P>,
}

impl<T, N: ArrayLength<T>, P: Unsigned> GenericArrayConsumer<T, N, P> {
    fn new(array: GenericArray<T, N>) -> GenericArrayConsumer<T, N, P> {
        GenericArrayConsumer {
            array: NoDrop::new(array),
            _position: PhantomData,
        }
    }

    #[inline(always)]
    fn cast<O: Unsigned>(self) -> GenericArrayConsumer<T, N, O> {
        GenericArrayConsumer {
            array: unsafe {
                let array = ptr::read(&self.array);

                mem::forget(self);

                array
            },
            _position: PhantomData,
        }
    }
}

impl<T, N: ArrayLength<T>, P: Unsigned> Drop for GenericArrayConsumer<T, N, P> {
    fn drop(&mut self) {
        for i in P::to_usize()..N::to_usize() {
            unsafe {
                ptr::drop_in_place(self.array.get_unchecked_mut(i));
            }
        }
    }
}

/// Internal trait for type-level recursion of array lengths from N to zero,
/// allowing for compile-time evaluation of array iteration.
pub trait IterableArrayLength<T, N: ArrayLength<T>>: ArrayLength<T> {
    #[doc(hidden)]
    type Index: ArrayLength<T>;

    #[doc(hidden)]
    fn generate<F>(builder: RecursiveArrayBuilder<T, N, Self::Index>, f: F) -> GenericArray<T, N>
    where
        F: Fn(usize) -> T;

    #[doc(hidden)]
    fn map<U, F>(
        builder: RecursiveArrayBuilder<T, N, Self::Index>,
        source: GenericArrayConsumer<U, N, Self::Index>,
        f: F,
    ) -> GenericArray<T, N>
    where
        F: Fn(U) -> T,
        N: ArrayLength<U>;

    #[doc(hidden)]
    fn zip<A, B, F>(
        builder: RecursiveArrayBuilder<T, N, Self::Index>,
        lhs: GenericArrayConsumer<A, N, Self::Index>,
        rhs: GenericArrayConsumer<B, N, Self::Index>,
        f: F,
    ) -> GenericArray<T, N>
    where
        F: Fn(A, B) -> T,
        N: ArrayLength<A> + ArrayLength<B>;
}

impl<T, N: ArrayLength<T>> IterableArrayLength<T, N> for UTerm {
    #[doc(hidden)]
    type Index = N;

    #[inline(always)]
    fn generate<F>(builder: RecursiveArrayBuilder<T, N, Self::Index>, _: F) -> GenericArray<T, N>
    where
        F: Fn(usize) -> T,
    {
        let array = unsafe { ptr::read(&builder.array) };

        mem::forget(builder);

        array.into_inner()
    }

    #[inline(always)]
    fn map<U, F>(
        builder: RecursiveArrayBuilder<T, N, Self::Index>,
        source: GenericArrayConsumer<U, N, Self::Index>,
        _: F,
    ) -> GenericArray<T, N>
    where
        F: Fn(U) -> T,
        N: ArrayLength<U>,
    {
        let array = unsafe { ptr::read(&builder.array) };

        mem::forget(source);
        mem::forget(builder);

        array.into_inner()
    }

    #[inline(always)]
    fn zip<A, B, F>(
        builder: RecursiveArrayBuilder<T, N, Self::Index>,
        lhs: GenericArrayConsumer<A, N, Self::Index>,
        rhs: GenericArrayConsumer<B, N, Self::Index>,
        _: F,
    ) -> GenericArray<T, N>
    where
        F: Fn(A, B) -> T,
        N: ArrayLength<A> + ArrayLength<B>,
    {
        let array = unsafe { ptr::read(&builder.array) };

        mem::forget(lhs);
        mem::forget(rhs);
        mem::forget(builder);

        array.into_inner()
    }
}

impl<T, N: ArrayLength<T>, U: Unsigned, B: Bit> IterableArrayLength<T, N> for UInt<U, B>
where
    Self: ArrayLength<T> + Sub<B1>,
    Sub1<Self>: IterableArrayLength<T, N>,
    N: Sub<UInt<U, B>>,
    <N as Sub<UInt<U, B>>>::Output: ArrayLength<T>,
{
    #[doc(hidden)]
    type Index = <N as Sub<UInt<U, B>>>::Output;

    #[inline(always)]
    fn generate<F>(builder: RecursiveArrayBuilder<T, N, Self::Index>, generator: F) -> GenericArray<T, N>
    where
        F: Fn(usize) -> T
    {
        <Sub1<Self> as IterableArrayLength<T, N>>::generate(RecursiveArrayBuilder {
            array: unsafe {
                let mut arr = ptr::read(&builder.array);

                ptr::write(arr.get_unchecked_mut(Self::Index::to_usize()),
                           generator(Self::Index::to_usize()));

                mem::forget(builder);

                arr
            },
            _position: PhantomData,
        }, generator)
    }

    #[inline]
    fn map<K, F>(
        builder: RecursiveArrayBuilder<T, N, Self::Index>,
        mut source: GenericArrayConsumer<K, N, Self::Index>,
        f: F
    ) -> GenericArray<T, N>
    where
        F: Fn(K) -> T,
        N: ArrayLength<K>
    {
        <Sub1<Self> as IterableArrayLength<T, N>>::map(RecursiveArrayBuilder {
            array: unsafe {
                let mut arr = ptr::read(&builder.array);

                ptr::write(arr.get_unchecked_mut(Self::Index::to_usize()), f(
                    ptr::read(source.array.get_unchecked_mut(Self::Index::to_usize()))
                ));

                mem::forget(builder);

                arr
            },
            _position: PhantomData,
        }, source.cast(), f)
    }

    #[inline]
    fn zip<A, K, F>(
        builder: RecursiveArrayBuilder<T, N, Self::Index>,
        mut lhs: GenericArrayConsumer<A, N, Self::Index>,
        mut rhs: GenericArrayConsumer<K, N, Self::Index>,
        f: F
    ) -> GenericArray<T, N>
    where
        F: Fn(A, K) -> T,
        N: ArrayLength<A> + ArrayLength<K>
    {
        <Sub1<Self> as IterableArrayLength<T, N>>::zip(RecursiveArrayBuilder {
            array: unsafe {
                let mut arr = ptr::read(&builder.array);

                ptr::write(arr.get_unchecked_mut(Self::Index::to_usize()), f(
                    ptr::read(lhs.array.get_unchecked_mut(Self::Index::to_usize())),
                    ptr::read(rhs.array.get_unchecked_mut(Self::Index::to_usize()))
                ));

                mem::forget(builder);

                arr
            },
            _position: PhantomData
        }, lhs.cast(), rhs.cast(), f)
    }
}
