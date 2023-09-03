use crate::*;

pub trait Sealed {}

impl<T> Sealed for [T; 0] {}

/// **UNSAFE**: Creates an array one element at a time using a mutable iterator of pointers.
///
/// You MUST increment the position while iterating to mark off created elements,
/// which will be dropped if `into_inner` is not called.
pub struct ArrayBuilder<T, N: ArrayLength> {
    array: GenericArray<MaybeUninit<T>, N>,
    position: usize,
}

impl<T, N: ArrayLength> ArrayBuilder<T, N> {
    /// Begin building an array
    #[inline(always)]
    pub const fn new() -> ArrayBuilder<T, N> {
        ArrayBuilder {
            array: GenericArray::uninit(),
            position: 0,
        }
    }

    /// Creates a mutable iterator for writing to the array elements.
    ///
    /// You MUST increment the position value (given as a mutable reference) as you iterate
    /// to mark how many elements have been created.
    ///
    /// ```
    /// #[cfg(feature = "internals")]
    /// # {
    /// # use generic_array::{GenericArray, internals::ArrayBuilder, typenum::U5};
    /// # struct SomeType;
    /// fn make_some_struct() -> SomeType { SomeType }
    /// unsafe {
    ///     let mut builder = ArrayBuilder::<SomeType, U5>::new();
    ///     let (dst_iter, position) = builder.iter_position();
    ///     for dst in dst_iter {
    ///         dst.write(make_some_struct());
    ///         // MUST be done AFTER ownership of the value has been given to `dst.write`
    ///         *position += 1;
    ///     }
    ///     let your_array = builder.into_inner();
    /// }
    /// # }
    /// ```
    #[inline(always)]
    pub unsafe fn iter_position(&mut self) -> (slice::IterMut<MaybeUninit<T>>, &mut usize) {
        (self.array.iter_mut(), &mut self.position)
    }

    /// When done writing (assuming all elements have been written to),
    /// get the inner array.
    #[inline(always)]
    pub unsafe fn into_inner(self) -> GenericArray<T, N> {
        debug_assert_eq!(self.position, N::USIZE);

        let array = ptr::read(&self.array);
        mem::forget(self);
        GenericArray::assume_init(array)
    }
}

impl<T, N: ArrayLength> Drop for ArrayBuilder<T, N> {
    fn drop(&mut self) {
        if mem::needs_drop::<T>() {
            unsafe {
                for value in &mut self.array[..self.position] {
                    value.assume_init_drop();
                }
            }
        }
    }
}

/// **UNSAFE**: Consumes an array one element at a time.
///
/// You MUST increment the position while iterating and any leftover elements
/// will be dropped if position does not go to N
pub struct ArrayConsumer<T, N: ArrayLength> {
    array: ManuallyDrop<GenericArray<T, N>>,
    position: usize,
}

impl<T, N: ArrayLength> ArrayConsumer<T, N> {
    /// Give ownership of the array to the consumer
    #[inline(always)]
    pub const fn new(array: GenericArray<T, N>) -> ArrayConsumer<T, N> {
        ArrayConsumer {
            array: ManuallyDrop::new(array),
            position: 0,
        }
    }

    /// Creates an iterator and mutable reference to the internal position
    /// to keep track of consumed elements.
    ///
    /// You MUST increment the position as you iterate to mark off consumed elements.
    #[inline(always)]
    pub unsafe fn iter_position(&mut self) -> (slice::Iter<T>, &mut usize) {
        (self.array.iter(), &mut self.position)
    }
}

impl<T, N: ArrayLength> Drop for ArrayConsumer<T, N> {
    fn drop(&mut self) {
        if mem::needs_drop::<T>() {
            for value in &mut self.array[self.position..N::USIZE] {
                unsafe {
                    ptr::drop_in_place(value);
                }
            }
        }
    }
}
