//! `GenericArray` iterator implementation.

use super::{ArrayLength, GenericArray};
use core::iter::FusedIterator;
use core::mem::ManuallyDrop;
use core::{cmp, fmt, mem, ptr};

/// Build a raw slice pointer over the live sub-range `array[start..end]` of an iterator's
/// backing `ManuallyDrop<GenericArray<T, N>>`, without ever forming a reference (not even a
/// transient `&[T]`) that spans the moved-out slots outside `start..end`.
///
/// The base pointer is taken via `addr_of!`/`addr_of_mut!` on the array place (the same raw
/// cast `GenericArray::as_slice` uses), so the only reference - if any - is the one the
/// *caller* materializes from the returned raw pointer at the use site.
///
/// Forms:
/// - `raw_subslice!(const $array, $start, $end)` -> `*const [T]`
/// - `raw_subslice!(mut   $array, $start, $end)` -> `*mut [T]`
///
/// # Safety
///
/// The caller must ensure `$start <= $end <= N` (the iterator's `index <= index_back <= N`
/// invariant) so the range is in bounds, and - for the `mut` form - that no other live
/// borrow of the array overlaps `$start..$end`.
macro_rules! raw_subslice {
    (const $array:expr, $start:expr, $end:expr) => {{
        let base = ::core::ptr::addr_of!(*$array) as *const T;
        ::core::ptr::slice_from_raw_parts(base.add($start), $end - $start)
    }};
    (mut $array:expr, $start:expr, $end:expr) => {{
        let base = ::core::ptr::addr_of_mut!(*$array) as *mut T;
        ::core::ptr::slice_from_raw_parts_mut(base.add($start), $end - $start)
    }};
}

/// An iterator that moves out of a [`GenericArray`]
pub struct GenericArrayIter<T, N: ArrayLength> {
    // Invariants: index <= index_back <= N
    // Only values in array[index..index_back] are alive at any given time.
    // Values from array[..index] and array[index_back..] are already moved/dropped.
    array: ManuallyDrop<GenericArray<T, N>>,
    index: usize,
    index_back: usize,
}

impl<T, N: ArrayLength> GenericArrayIter<T, N> {
    /// Raw `*const T` to element 0 of the backing array.
    ///
    /// Taken via a raw cast of the array's address (the same cast `GenericArray::as_slice`
    /// uses), so that callers offsetting into the live range never form a reference - not
    /// even a transient `&[T]` - spanning the moved-out slots in `..index`/`index_back..`.
    #[inline(always)]
    fn base_ptr(&self) -> *const T {
        ptr::addr_of!(*self.array) as *const T
    }

    /// Returns the remaining items of this iterator as a slice
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: By the type invariant, `index <= index_back <= N`, so `index` is in
        // bounds and `index_back - index` does not exceed the backing allocation. We form
        // the slice directly over the live `index..index_back` range rather than letting
        // `GenericArray`'s `Deref` materialize a `&[T]` spanning all `0..N` and then
        // re-slicing: this way no reference ever spans the moved-out slots in `..index`
        // or `index_back..` (same discipline as `core::array::IntoIter::as_slice`). The
        // live elements are initialized and valid for reads. (Codegen is identical to the
        // old `get_unchecked` form; see `examples/asm.rs`.)
        unsafe { &*raw_subslice!(const self.array, self.index, self.index_back) }
    }

    /// Returns the remaining items of this iterator as a mutable slice
    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: By the type invariant, `index <= index_back <= N`, so `index` is in
        // bounds and the length `index_back - index` does not exceed the backing
        // allocation. The slice spans only the live range, so it never references the
        // moved-out slots, and uniqueness holds because it is derived from `&mut self`.
        unsafe { &mut *raw_subslice!(mut self.array, self.index, self.index_back) }
    }
}

impl<T, N: ArrayLength> IntoIterator for GenericArray<T, N> {
    type Item = T;
    type IntoIter = GenericArrayIter<T, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        GenericArrayIter {
            array: ManuallyDrop::new(self),
            index: 0,
            index_back: N::USIZE,
        }
    }
}

// Based on work in rust-lang/rust#49000
impl<T: fmt::Debug, N: ArrayLength> fmt::Debug for GenericArrayIter<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("GenericArrayIter")
            .field(&self.as_slice())
            .finish()
    }
}

impl<T, N: ArrayLength> Drop for GenericArrayIter<T, N> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.as_mut_slice());
        }
    }
}

// Based on work in rust-lang/rust#49000
impl<T: Clone, N: ArrayLength> Clone for GenericArrayIter<T, N> {
    fn clone(&self) -> Self {
        // This places all cloned elements at the start of the new array iterator,
        // not at their original indices.

        // work from within the iter so if clone panics then the iter will drop itself.
        let mut iter = GenericArrayIter {
            array: unsafe { ptr::read(&self.array) },
            index: 0,
            index_back: 0,
        };

        for (dst, src) in iter.array.as_mut_slice().iter_mut().zip(self.as_slice()) {
            unsafe { ptr::write(dst, src.clone()) };
            iter.index_back += 1;
        }

        iter
    }
}

impl<T, N: ArrayLength> Iterator for GenericArrayIter<T, N> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.index < self.index_back {
            // SAFETY: `index < index_back <= N`, so element `index` is in bounds, alive,
            // and not yet read. Read it by value through the base pointer (no reference to
            // a moved-out slot is ever formed); `index` is advanced past it immediately.
            let p = unsafe { Some(ptr::read(self.base_ptr().add(self.index))) };

            self.index += 1;

            p
        } else {
            None
        }
    }

    #[inline]
    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        let ret = unsafe {
            let GenericArrayIter {
                ref array,
                ref mut index,
                index_back,
            } = self;

            // SAFETY: `index <= index_back <= N`. Form a slice over only the live
            // `index..index_back` range (raw cast of the array address, so no reference
            // ever spans the moved-out slots), then read each element out by value exactly
            // once, advancing `index` so a panic in `f` drops only the tail.
            let remaining = &*raw_subslice!(const *array, *index, index_back);

            remaining.iter().fold(init, |acc, src| {
                let value = ptr::read(src);

                *index += 1;

                f(acc, value)
            })
        };

        // The current iterator is now empty after the remaining items are
        // consumed by the above folding. Dropping it is unnecessary,
        // so avoid the drop codegen and forget it instead. The iterator
        // will still drop on panics from `f`, of course.
        //
        // Furthermore, putting `forget` here at the end ensures the above
        // destructuring never moves by value, so its behavior on drop remains intact.
        mem::forget(self);

        ret
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline(always)]
    fn count(self) -> usize {
        self.len()
    }

    fn nth(&mut self, n: usize) -> Option<T> {
        // First consume values prior to the nth.
        let next_index = self.index + cmp::min(n, self.len());

        // SAFETY: `index <= next_index <= index_back <= N`, so `index..next_index` is a
        // range of live, in-bounds elements. Drop exactly those in place via a raw slice
        // pointer (no reference spanning moved-out slots), then advance `index` past them.
        unsafe {
            ptr::drop_in_place(raw_subslice!(mut self.array, self.index, next_index));
        }

        self.index = next_index;

        self.next()
    }

    #[inline]
    fn last(mut self) -> Option<T> {
        // Note, everything else will correctly drop first as `self` leaves scope.
        self.next_back()
    }
}

impl<T, N: ArrayLength> DoubleEndedIterator for GenericArrayIter<T, N> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        if self.index < self.index_back {
            self.index_back -= 1;

            // SAFETY: after the decrement, `index <= index_back < N`, so element
            // `index_back` is in bounds, alive, and not yet read. Read it by value through
            // the base pointer; `index_back` already excludes it from the live range.
            unsafe { Some(ptr::read(self.base_ptr().add(self.index_back))) }
        } else {
            None
        }
    }

    #[inline]
    fn rfold<B, F>(mut self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        let ret = unsafe {
            let GenericArrayIter {
                ref array,
                index,
                ref mut index_back,
            } = self;

            // SAFETY: `index <= index_back <= N`. Form a slice over only the live
            // `index..index_back` range (raw cast of the array address, so no reference
            // spans moved-out slots), then read each element out by value exactly once from
            // the back, decrementing `index_back` so a panic in `f` drops only the head.
            let remaining = &*raw_subslice!(const *array, index, *index_back);

            remaining.iter().rfold(init, |acc, src| {
                let value = ptr::read(src);

                *index_back -= 1;

                f(acc, value)
            })
        };

        // Same as `fold`
        mem::forget(self);

        ret
    }

    fn nth_back(&mut self, n: usize) -> Option<T> {
        let next_back = self.index_back - cmp::min(n, self.len());

        // SAFETY: `index <= next_back <= index_back <= N`, so `next_back..index_back` is a
        // range of live, in-bounds elements. Drop exactly those in place via a raw slice
        // pointer (no reference spanning moved-out slots), then retreat `index_back`.
        unsafe {
            ptr::drop_in_place(raw_subslice!(mut self.array, next_back, self.index_back));
        }

        self.index_back = next_back;

        self.next_back()
    }
}

impl<T, N: ArrayLength> ExactSizeIterator for GenericArrayIter<T, N> {
    #[inline]
    fn len(&self) -> usize {
        self.index_back - self.index
    }
}

impl<T, N: ArrayLength> FusedIterator for GenericArrayIter<T, N> {}

// TODO: Implement `TrustedLen` when stabilized

#[cfg(test)]
mod test {
    use super::*;

    fn send<I: Send>(_iter: I) {}

    #[test]
    fn test_send_iter() {
        send(GenericArray::from([1, 2, 3, 4]).into_iter());
    }
}
