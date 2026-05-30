#![no_std]
use core::cell::Cell;
use core::ops::{Add, Drop};
use generic_array::arr;
use generic_array::functional::*;
use generic_array::sequence::*;
use generic_array::typenum::{U0, U3, U4, U97};
use generic_array::GenericArray;

#[test]
fn test() {
    let mut list97 = [0; 97];
    for (i, elem) in list97.iter_mut().enumerate() {
        *elem = i as i32;
    }
    let l: GenericArray<i32, U97> = *GenericArray::from_slice(&list97);
    assert_eq!(l[0], 0);
    assert_eq!(l[1], 1);
    assert_eq!(l[32], 32);
    assert_eq!(l[56], 56);
}

#[test]
fn test_drop() {
    #[derive(Clone)]
    struct TestDrop<'a>(&'a Cell<u32>);

    impl<'a> Drop for TestDrop<'a> {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let drop_counter = Cell::new(0);
    {
        let _: GenericArray<TestDrop, U3> = arr![
            TestDrop(&drop_counter),
            TestDrop(&drop_counter),
            TestDrop(&drop_counter)
        ];
    }
    assert_eq!(drop_counter.get(), 3);
}

#[test]
fn test_arr() {
    let test: GenericArray<u32, U3> = arr![1, 2, 3];
    assert_eq!(test[1], 2);
}

#[test]
fn test_copy() {
    let test = arr![1, 2, 3];
    let test2 = test;
    // if GenericArray is not copy, this should fail as a use of a moved value
    assert_eq!(test[1], 2);
    assert_eq!(test2[0], 1);
}

#[derive(Debug, PartialEq, Eq)]
struct NoClone<T>(T);

#[test]
fn test_from_slice() {
    let arr = [1, 2, 3, 4];
    let gen_arr = GenericArray::<_, U3>::from_slice(&arr[..3]);
    assert_eq!(&arr[..3], gen_arr.as_slice());
    let arr = [NoClone(1u32), NoClone(2), NoClone(3), NoClone(4)];
    let gen_arr = GenericArray::<_, U3>::from_slice(&arr[..3]);
    assert_eq!(&arr[..3], gen_arr.as_slice());
}

#[test]
fn test_from_mut_slice() {
    let mut arr = [1, 2, 3, 4];
    {
        let gen_arr = GenericArray::<_, U3>::from_mut_slice(&mut arr[..3]);
        gen_arr[2] = 10;
    }
    assert_eq!(arr, [1, 2, 10, 4]);
    let mut arr = [NoClone(1u32), NoClone(2), NoClone(3), NoClone(4)];
    {
        let gen_arr = GenericArray::<_, U3>::from_mut_slice(&mut arr[..3]);
        gen_arr[2] = NoClone(10);
    }
    assert_eq!(arr, [NoClone(1), NoClone(2), NoClone(10), NoClone(4)]);
}

#[test]
fn test_default() {
    let arr = GenericArray::<u8, U4>::default();
    assert_eq!(arr.as_slice(), &[0, 0, 0, 0]);
}

#[test]
fn test_from() {
    let data = [(1, 2, 3), (4, 5, 6), (7, 8, 9)];
    let garray: GenericArray<(usize, usize, usize), U3> = data.into();
    assert_eq!(&data, garray.as_slice());
}

#[test]
fn test_unit_macro() {
    let arr = arr![5.81];
    assert_eq!(arr[0], 5.81);
}

#[test]
fn test_empty_macro() {
    let _arr: GenericArray<(), _> = arr![];
}

#[test]
fn test_cmp() {
    let _ = arr![0x00u8].cmp(&arr![0x00]);
}

/// This test should cause a helpful compile error if uncommented.
// #[test]
// fn test_empty_macro2(){
//     let arr = arr![];
// }
#[cfg(feature = "serde")]
mod impl_serde {
    extern crate serde_json;

    use generic_array::arr;
    use generic_array::typenum::U6;
    use generic_array::GenericArray;

    #[test]
    fn test_serde_implementation() {
        let array: GenericArray<f64, U6> = arr![0.0, 5.0, 3.0, 7.07192, 76.0, -9.0];
        let string = serde_json::to_string(&array).unwrap();
        assert_eq!(string, "[0.0,5.0,3.0,7.07192,76.0,-9.0]");

        let test_array: GenericArray<f64, U6> = serde_json::from_str(&string).unwrap();
        assert_eq!(test_array, array);
    }
}

#[test]
fn test_map() {
    let b: GenericArray<i32, U4> = GenericArray::generate(|i| i as i32 * 4).map(|x| x - 3);

    assert_eq!(b, arr![-3, 1, 5, 9]);
}

#[test]
fn test_zip() {
    let a: GenericArray<_, U4> = GenericArray::generate(|i| i + 1);
    let b: GenericArray<_, U4> = GenericArray::generate(|i| i as i32 * 4);

    // Uses reference and non-reference arguments
    let c = (&a).zip(b, |r, l| *r as i32 + l);

    assert_eq!(c, arr![1, 6, 11, 16]);
}

#[test]
#[should_panic]
fn test_from_iter_short() {
    use core::iter::repeat;

    let a: GenericArray<_, U4> = repeat(11).take(3).collect();

    assert_eq!(a, arr![11, 11, 11, 0]);
}

#[test]
fn test_from_iter() {
    use core::iter::{once, repeat};

    let a: GenericArray<_, U4> = repeat(11).take(3).chain(once(0)).collect();

    assert_eq!(a, arr![11, 11, 11, 0]);
}

#[allow(unused)]
#[derive(Debug, Copy, Clone)]
enum E {
    V,
    V2(i32),
    V3 { h: bool, i: i32 },
}

#[allow(unused)]
#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(packed)]
struct Test {
    t: u16,
    s: u32,
    mm: bool,
    r: u16,
    f: u16,
    p: (),
    o: u32,
    ff: *const extern "C" fn(*const char) -> *const core::ffi::c_void,
    l: *const core::ffi::c_void,
    w: bool,
    q: bool,
    v: E,
}

#[test]
fn test_sizes() {
    use core::mem::{size_of, size_of_val};

    assert_eq!(size_of::<E>(), 8);

    assert_eq!(size_of::<Test>(), 25 + size_of::<usize>() * 2);

    assert_eq!(size_of_val(&arr![1u8, 2, 3]), size_of::<u8>() * 3);
    assert_eq!(size_of_val(&arr![1u32]), size_of::<u32>() * 1);
    assert_eq!(size_of_val(&arr![1u64, 2, 3, 4]), size_of::<u64>() * 4);

    assert_eq!(size_of::<GenericArray<Test, U97>>(), size_of::<Test>() * 97);
}

#[test]
fn test_alignment() {
    use core::mem::align_of;

    assert_eq!(
        align_of::<GenericArray::<u32, U0>>(),
        align_of::<[u32; 0]>()
    );
    assert_eq!(
        align_of::<GenericArray::<u32, U3>>(),
        align_of::<[u32; 3]>()
    );
    assert_eq!(
        align_of::<GenericArray::<Test, U3>>(),
        align_of::<[Test; 3]>()
    );
}

#[test]
fn test_append() {
    let a = arr![1, 2, 3];

    let b = a.append(4);

    assert_eq!(b, arr![1, 2, 3, 4]);
}

#[test]
fn test_prepend() {
    let a = arr![1, 2, 3];

    let b = a.prepend(4);

    assert_eq!(b, arr![4, 1, 2, 3]);
}

#[test]
fn test_pop() {
    let a = arr![1, 2, 3, 4];

    let (init, last) = a.pop_back();

    assert_eq!(init, arr![1, 2, 3]);
    assert_eq!(last, 4);

    let (head, tail) = a.pop_front();

    assert_eq!(head, 1);
    assert_eq!(tail, arr![2, 3, 4]);
}

#[test]
fn test_split() {
    let a = arr![1, 2, 3, 4];

    let (b, c) = a.split();

    assert_eq!(b, arr![1]);
    assert_eq!(c, arr![2, 3, 4]);

    let (e, f) = a.split();

    assert_eq!(e, arr![1, 2]);
    assert_eq!(f, arr![3, 4]);
}

#[test]
fn test_split_ref() {
    let a = arr![1, 2, 3, 4];
    let a_ref = &a;

    let (b_ref, c_ref) = a_ref.split();

    assert_eq!(b_ref, &arr![1]);
    assert_eq!(c_ref, &arr![2, 3, 4]);

    let (e_ref, f_ref) = a_ref.split();

    assert_eq!(e_ref, &arr![1, 2]);
    assert_eq!(f_ref, &arr![3, 4]);
}

#[test]
fn test_split_mut() {
    let mut a = arr![1, 2, 3, 4];
    let a_ref = &mut a;

    let (b_ref, c_ref) = a_ref.split();

    assert_eq!(b_ref, &mut arr![1]);
    assert_eq!(c_ref, &mut arr![2, 3, 4]);

    let (e_ref, f_ref) = a_ref.split();

    assert_eq!(e_ref, &mut arr![1, 2]);
    assert_eq!(f_ref, &mut arr![3, 4]);
}

#[test]
fn test_concat() {
    let a = arr![1, 2];
    let b = arr![3, 4, 5];

    let c = a.concat(b);

    assert_eq!(c, arr![1, 2, 3, 4, 5]);

    let (d, e) = c.split();

    assert_eq!(d, arr![1, 2]);
    assert_eq!(e, arr![3, 4, 5]);
}

#[test]
fn test_removes() {
    let a = arr![1, 2, 3, 4];

    for i in 0..4 {
        let (b, c) = a.remove(i);

        assert_eq!(b, i + 1);
        assert_eq!(
            c,
            match i {
                0 => arr![2, 3, 4],
                1 => arr![1, 3, 4],
                2 => arr![1, 2, 4],
                3 => arr![1, 2, 3],
                _ => unreachable!(),
            }
        );

        let (b, c) = a.swap_remove(i);

        assert_eq!(b, i + 1);
        assert_eq!(
            c,
            match i {
                0 => arr![4, 2, 3],
                1 => arr![1, 4, 3],
                2 => arr![1, 2, 4],
                3 => arr![1, 2, 3],
                _ => unreachable!(),
            }
        );
    }
}

#[test]
fn test_fold() {
    let a = arr![1, 2, 3, 4];

    assert_eq!(10, a.fold(0, |a, x| a + x));
}

fn sum_generic<S>(s: S) -> i32
where
    S: FunctionalSequence<i32>,
    S::Item: Add<i32, Output = i32>, // `+`
    i32: Add<S::Item, Output = i32>, // reflexive
{
    s.fold(0, |a, x| a + x)
}

#[test]
fn test_sum() {
    let a = sum_generic(arr![1, 2, 3, 4]);

    assert_eq!(a, 10);
}

#[test]
fn test_as_ref() {
    let a = arr![1, 2, 3, 4];
    let a_ref: &[i32; 4] = a.as_ref();
    assert_eq!(a_ref, &[1, 2, 3, 4]);
}

#[test]
fn test_as_mut() {
    let mut a = arr![1, 2, 3, 4];
    let a_mut: &mut [i32; 4] = a.as_mut();
    assert_eq!(a_mut, &mut [1, 2, 3, 4]);
    a_mut[2] = 0;
    assert_eq!(a_mut, &mut [1, 2, 0, 4]);
    assert_eq!(a, arr![1, 2, 0, 4]);
}

#[test]
fn test_from_array_ref() {
    let a = arr![1, 2, 3, 4];
    let a_ref: &[i32; 4] = a.as_ref();
    let a_from: &GenericArray<i32, U4> = a_ref.into();
    assert_eq!(&a, a_from);
}

#[test]
fn test_from_array_mut() {
    let mut a = arr![1, 2, 3, 4];
    let mut a_copy = a;
    let a_mut: &mut [i32; 4] = a.as_mut();
    let a_from: &mut GenericArray<i32, U4> = a_mut.into();
    assert_eq!(&mut a_copy, a_from);
}

#[test]
fn test_chunks() {
    // intended usage
    let (chunks, rem) = GenericArray::<u8, U3>::chunks_from_slice(&[1, 2, 3, 4, 5, 6, 7]);

    assert_eq!(chunks[0], arr![1, 2, 3]);
    assert_eq!(chunks[1], arr![4, 5, 6]);
    assert_eq!(rem, &[7]);

    // zero-length input
    let (chunks, rem) = GenericArray::<u8, U3>::chunks_from_slice(&[]);
    assert!(chunks.is_empty());
    assert!(rem.is_empty());

    // zero-length output with zero-length input
    let (chunks, rem) = GenericArray::<u8, U0>::chunks_from_slice(&[]);
    assert!(chunks.is_empty());
    assert!(rem.is_empty());

    // only remainder
    let (chunks, rem) = GenericArray::<u8, U3>::chunks_from_slice(&[1, 2]);

    assert!(chunks.is_empty());
    assert_eq!(rem, &[1, 2]);
}

#[test]
#[should_panic]
fn test_chunks_fail() {
    // zero-length output with input
    let (chunks, rem) = GenericArray::<u8, U0>::chunks_from_slice(&[1, 2, 3]);
    assert!(chunks.is_empty());
    assert!(rem.is_empty());
}

#[test]
fn test_try_map() {
    let a = arr![1, 2, 3, 4];

    let b = a.try_map(|x| {
        if x % 2 == 0 {
            Ok(x * 2)
        } else {
            Err("odd number")
        }
    });

    assert!(b.is_err());
}

#[test]
fn test_try_generate_success() {
    use generic_array::typenum::U0;

    let a: GenericArray<i32, U4> = GenericArray::try_generate(|i| Ok::<_, ()>(i as i32 * 2))
        .unwrap()
        .unwrap();
    assert_eq!(a, arr![0, 2, 4, 6]);

    // zero-length always succeeds
    let _: GenericArray<i32, U0> = GenericArray::try_generate(|_| Ok::<_, ()>(0))
        .unwrap()
        .unwrap();
}

#[test]
fn test_try_generate_error() {
    // generator fails on the 3rd element (index 2)
    let result: Result<Result<GenericArray<i32, U4>, &str>, _> =
        GenericArray::try_generate(|i| if i == 2 { Err("bad") } else { Ok(i as i32) });

    assert!(
        result.is_ok(),
        "outer Result should be Ok (no alloc failure)"
    );
    assert_eq!(result.unwrap(), Err("bad"));
}

#[test]
fn test_try_generate_drops_on_error() {
    #[derive(Clone)]
    struct Tracked<'a>(&'a Cell<u32>);

    impl<'a> Drop for Tracked<'a> {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let counter = Cell::new(0u32);
    // generator succeeds for indices 0..3 then fails at index 3; the 3 initialized
    // elements must be dropped, the partially-init slot at index 3 must NOT be dropped
    let result: Result<Result<GenericArray<Tracked<'_>, U4>, &str>, _> =
        GenericArray::try_generate(|i| {
            if i == 3 {
                Err("fail")
            } else {
                Ok(Tracked(&counter))
            }
        });
    assert!(result.unwrap().is_err());
    assert_eq!(
        counter.get(),
        3,
        "exactly 3 initialized elements should be dropped"
    );
}

// Exercises the partial-init drop path in GenericArray::try_generate.
// Miri verifies that drop_in_place is called on exactly the initialized prefix
// and that no uninitialized memory is read.
#[test]
fn miri_try_generate_drop_on_error() {
    use core::cell::Cell;

    struct Tracked<'a>(&'a Cell<u32>);
    impl Drop for Tracked<'_> {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let counter = Cell::new(0u32);

    // error at index 0: no elements initialized, builder drop is a no-op slice
    let r: Result<Result<GenericArray<Tracked<'_>, U4>, ()>, _> =
        GenericArray::try_generate(|_| Err(()));
    assert!(r.unwrap().is_err());
    assert_eq!(counter.get(), 0);

    // error at index 2: two elements were initialized and must be dropped
    let r: Result<Result<GenericArray<Tracked<'_>, U4>, &str>, _> =
        GenericArray::try_generate(|i| {
            if i == 2 {
                Err("stop")
            } else {
                Ok(Tracked(&counter))
            }
        });
    assert!(r.unwrap().is_err());
    assert_eq!(counter.get(), 2);
}

// Exercises the ZST (U0) path in GenericArray::try_generate.
// The builder uses a dangling pointer for zero-sized arrays; Miri verifies
// no invalid pointer operations occur.
#[test]
fn miri_try_generate_zero_length() {
    let r: Result<Result<GenericArray<u32, U0>, ()>, _> = GenericArray::try_generate(|_| Err(()));
    // generator is never called for N=0, so the result is Ok(Ok(...))
    assert!(r.unwrap().is_ok());

    let arr: GenericArray<u32, U0> = GenericArray::try_generate(|_| Ok::<_, ()>(0))
        .unwrap()
        .unwrap();
    assert_eq!(arr.len(), 0);
}

/// A `Drop`-counting element used to verify that the `mem::needs_drop` branches
/// in `zip`/`fold`/`inverted_zip` actually run their destructors exactly once
/// per element. `i32`-based tests only ever hit the no-drop fast paths.
#[derive(Clone, PartialEq, Eq, Debug)]
struct Tracked<'a>(i32, &'a Cell<u32>);

impl Drop for Tracked<'_> {
    fn drop(&mut self) {
        self.1.set(self.1.get() + 1);
    }
}

#[test]
fn test_partial_ord_and_borrow() {
    use core::borrow::{Borrow, BorrowMut};

    let a = arr![1, 2, 3, 4];
    let b = arr![1, 2, 4, 4];

    // PartialOrd / Ord
    assert!(a < b);
    assert_eq!(a.partial_cmp(&b), Some(core::cmp::Ordering::Less));
    assert_eq!(a.cmp(&a), core::cmp::Ordering::Equal);

    // Borrow / BorrowMut to [T]
    let borrowed: &[i32] = a.borrow();
    assert_eq!(borrowed, &[1, 2, 3, 4]);

    let mut c = arr![1, 2, 3, 4];
    let borrowed_mut: &mut [i32] = c.borrow_mut();
    borrowed_mut[0] = 9;
    assert_eq!(c, arr![9, 2, 3, 4]);

    // AsRef / AsMut to [T] (slice, not [T; N])
    let as_ref: &[i32] = AsRef::<[i32]>::as_ref(&a);
    assert_eq!(as_ref, &[1, 2, 3, 4]);
    let mut d = arr![1, 2, 3, 4];
    AsMut::<[i32]>::as_mut(&mut d)[1] = 0;
    assert_eq!(d, arr![1, 0, 3, 4]);
}

#[test]
fn test_tuple_conversions() {
    use generic_array::typenum::U3;

    let from_tuple: GenericArray<i32, U3> = (1, 2, 3).into();
    assert_eq!(from_tuple, arr![1, 2, 3]);

    let back: (i32, i32, i32) = from_tuple.into();
    assert_eq!(back, (1, 2, 3));
}

#[test]
fn test_each_ref_mut() {
    let a = arr![1, 2, 3, 4];

    let refs: GenericArray<&i32, _> = a.each_ref();
    assert_eq!(refs, arr![&1, &2, &3, &4]);

    let mut b = arr![1, 2, 3, 4];
    for r in b.each_mut() {
        *r *= 10;
    }
    assert_eq!(b, arr![10, 20, 30, 40]);
}

#[test]
fn test_chunks_from_slice_mut() {
    use generic_array::typenum::U3;

    let mut data = [1u8, 2, 3, 4, 5, 6, 7];
    let (chunks, rem) = GenericArray::<u8, U3>::chunks_from_slice_mut(&mut data);

    assert_eq!(chunks.len(), 2);
    chunks[0][0] = 100;
    chunks[1][2] = 200;
    assert_eq!(rem, &mut [7]);
    rem[0] = 0;

    assert_eq!(data, [100, 2, 3, 4, 5, 200, 0]);
}

#[test]
fn test_slice_from_chunks() {
    use generic_array::typenum::U3;

    let mut chunks = [arr![1u8, 2, 3], arr![4, 5, 6]];

    let flat = GenericArray::<u8, U3>::slice_from_chunks(&chunks);
    assert_eq!(flat, &[1, 2, 3, 4, 5, 6]);

    let flat_mut = GenericArray::<u8, U3>::slice_from_chunks_mut(&mut chunks);
    flat_mut[0] = 9;
    assert_eq!(chunks[0], arr![9, 2, 3]);
}

#[test]
fn test_from_into_chunks() {
    use generic_array::typenum::U3;

    let mut native = [[1u8, 2, 3], [4, 5, 6]];

    let as_ga = GenericArray::<u8, U3>::from_chunks(&native);
    assert_eq!(as_ga, &[arr![1, 2, 3], arr![4, 5, 6]]);

    let as_ga_mut = GenericArray::<u8, U3>::from_chunks_mut(&mut native);
    as_ga_mut[0][0] = 7;
    assert_eq!(native[0], [7, 2, 3]);

    let ga_chunks = [arr![1u8, 2, 3], arr![4, 5, 6]];
    let back: &[[u8; 3]] = GenericArray::<u8, U3>::into_chunks(&ga_chunks);
    assert_eq!(back, &[[1, 2, 3], [4, 5, 6]]);

    let mut ga_chunks_mut = [arr![1u8, 2, 3], arr![4, 5, 6]];
    let back_mut: &mut [[u8; 3]] = GenericArray::<u8, U3>::into_chunks_mut(&mut ga_chunks_mut);
    back_mut[1][0] = 0;
    assert_eq!(ga_chunks_mut[1], arr![0, 5, 6]);
}

#[test]
fn test_as_array_of_cells() {
    let cell = Cell::new(arr![1, 2, 3, 4]);

    let cells = GenericArray::as_array_of_cells(&cell);
    cells[2].set(30);

    assert_eq!(cell.into_inner(), arr![1, 2, 30, 4]);
}

#[test]
fn test_try_from_slice_variants() {
    use core::convert::TryFrom;
    use generic_array::typenum::U3;

    // try_from_slice: Ok and Err
    let ok = GenericArray::<i32, U3>::try_from_slice(&[1, 2, 3]);
    assert_eq!(ok.unwrap(), &arr![1, 2, 3]);
    assert!(GenericArray::<i32, U3>::try_from_slice(&[1, 2]).is_err());

    // try_from_mut_slice: Ok and Err
    let mut data = [1, 2, 3];
    {
        let ga = GenericArray::<i32, U3>::try_from_mut_slice(&mut data).unwrap();
        ga[0] = 9;
    }
    assert_eq!(data, [9, 2, 3]);
    let mut short = [1, 2];
    assert!(GenericArray::<i32, U3>::try_from_mut_slice(&mut short).is_err());

    // TryFrom impls for the reference types
    let r: Result<&GenericArray<i32, U3>, _> = <&GenericArray<i32, U3>>::try_from(&[1, 2, 3][..]);
    assert_eq!(r.unwrap(), &arr![1, 2, 3]);
    let mut data2 = [1, 2, 3];
    let rm: Result<&mut GenericArray<i32, U3>, _> =
        <&mut GenericArray<i32, U3>>::try_from(&mut data2[..]);
    assert!(rm.is_ok());
    assert!(<&GenericArray<i32, U3>>::try_from(&[1, 2][..]).is_err());
}

#[test]
fn test_flatten_unflatten() {
    use generic_array::sequence::{Flatten, Unflatten};

    // owned
    let nested = arr![arr![1, 2], arr![3, 4], arr![5, 6]];
    assert_eq!(nested.flatten(), arr![1, 2, 3, 4, 5, 6]);

    let flat = arr![1, 2, 3, 4, 5, 6];
    assert_eq!(flat.unflatten(), arr![arr![1, 2], arr![3, 4], arr![5, 6]]);

    // by shared reference (UFCS to select the &T impl rather than the owned one)
    assert_eq!(Flatten::flatten(&nested), &arr![1, 2, 3, 4, 5, 6]);
    assert_eq!(
        Unflatten::unflatten(&flat),
        &arr![arr![1, 2], arr![3, 4], arr![5, 6]]
    );

    // by mutable reference
    let mut nested_mut = arr![arr![1, 2], arr![3, 4], arr![5, 6]];
    Flatten::flatten(&mut nested_mut)[0] = 100;
    assert_eq!(nested_mut[0], arr![100, 2]);

    let mut flat_mut = arr![1, 2, 3, 4, 5, 6];
    Unflatten::unflatten(&mut flat_mut)[0] = arr![7, 8];
    assert_eq!(flat_mut, arr![7, 8, 3, 4, 5, 6]);
}

#[test]
fn test_sequence_repeat() {
    use generic_array::typenum::U4;

    let a: GenericArray<i32, U4> = GenericArray::repeat(7);
    assert_eq!(a, arr![7, 7, 7, 7]);

    // The last element takes ownership; earlier elements are clones.
    let counter = Cell::new(0);
    {
        let repeated: GenericArray<Tracked, U4> = GenericArray::repeat(Tracked(1, &counter));
        assert_eq!(repeated, arr![Tracked(1, &counter), Tracked(1, &counter), Tracked(1, &counter), Tracked(1, &counter)]);
    }
}

#[test]
fn test_into_iter_mut() {
    let mut a = arr![1, 2, 3, 4];
    for x in &mut a {
        *x += 1;
    }
    assert_eq!(a, arr![2, 3, 4, 5]);
}

#[test]
fn test_zip_drop_path() {
    use generic_array::typenum::U4;

    // zip -> inverted_zip2, drop branch (rhs element needs Drop)
    let counter = Cell::new(0);
    {
        let a: GenericArray<Tracked, U4> =
            GenericArray::generate(|i| Tracked(i as i32, &counter));
        let b: GenericArray<Tracked, U4> =
            GenericArray::generate(|i| Tracked(i as i32 * 10, &counter));

        let summed: GenericArray<i32, U4> = a.zip(b, |x, y| x.0 + y.0);
        assert_eq!(summed, arr![0, 11, 22, 33]);
    }
    // 8 source elements consumed by the closure and dropped there.
    assert_eq!(counter.get(), 8);
}

#[test]
fn test_inverted_zip_drop_and_copy() {
    use generic_array::sequence::GenericSequence;
    use generic_array::typenum::U4;

    // Copy path (no drop)
    let lhs = arr![1, 2, 3, 4];
    let rhs = arr![10, 20, 30, 40];
    let c: GenericArray<i32, U4> = rhs.inverted_zip(lhs, |l, r| l + r);
    assert_eq!(c, arr![11, 22, 33, 44]);

    // Drop path
    let counter = Cell::new(0);
    {
        let lhs: GenericArray<Tracked, U4> =
            GenericArray::generate(|i| Tracked(i as i32, &counter));
        let rhs: GenericArray<Tracked, U4> =
            GenericArray::generate(|i| Tracked(i as i32, &counter));
        let _: GenericArray<i32, U4> = rhs.inverted_zip(lhs, |l, r| l.0 + r.0);
    }
    assert_eq!(counter.get(), 8);
}

#[test]
fn test_fold_and_try_fold_drop() {
    use generic_array::typenum::U4;

    // owned fold consuming Drop elements
    let counter = Cell::new(0);
    {
        let a: GenericArray<Tracked, U4> =
            GenericArray::generate(|i| Tracked(i as i32, &counter));
        let sum = a.fold(0, |acc, x| acc + x.0);
        assert_eq!(sum, 6);
    }
    assert_eq!(counter.get(), 4);

    // owned try_fold, success
    let counter = Cell::new(0);
    {
        let a: GenericArray<Tracked, U4> =
            GenericArray::generate(|i| Tracked(i as i32, &counter));
        let sum: Result<i32, ()> = a.try_fold(0, |acc, x| Ok(acc + x.0));
        assert_eq!(sum, Ok(6));
    }
    assert_eq!(counter.get(), 4);

    // owned try_fold, error mid-way still drops remaining source elements
    let counter = Cell::new(0);
    {
        let a: GenericArray<Tracked, U4> =
            GenericArray::generate(|i| Tracked(i as i32, &counter));
        let res: Result<i32, &str> = a.try_fold(0, |acc, x| {
            if x.0 == 2 {
                Err("stop")
            } else {
                Ok(acc + x.0)
            }
        });
        assert_eq!(res, Err("stop"));
    }
    assert_eq!(counter.get(), 4);
}

#[test]
fn test_functional_on_references() {
    use generic_array::typenum::U4;

    let a = arr![1, 2, 3, 4];

    // map on &GenericArray
    let doubled: GenericArray<i32, U4> = (&a).map(|x| x * 2);
    assert_eq!(doubled, arr![2, 4, 6, 8]);

    // fold on &GenericArray
    let sum = (&a).fold(0, |acc, x| acc + x);
    assert_eq!(sum, 10);

    // zip with a reference rhs -> trait-default inverted_zip2
    let b = arr![10, 20, 30, 40];
    let c: GenericArray<i32, U4> = a.zip(&b, |x, y| x + y);
    assert_eq!(c, arr![11, 22, 33, 44]);

    // map / fold on &mut GenericArray
    let mut d = arr![1, 2, 3, 4];
    let mapped: GenericArray<i32, U4> = (&mut d).map(|x| *x + 1);
    assert_eq!(mapped, arr![2, 3, 4, 5]);
    let s = (&mut d).fold(0, |acc, x| acc + *x);
    assert_eq!(s, 10);
}

#[test]
fn test_try_map_success() {
    let a = arr![2, 4, 6, 8];
    let b: Result<GenericArray<i32, _>, &str> = a.try_map(|x| {
        if x % 2 == 0 {
            Ok(x * 2)
        } else {
            Err("odd")
        }
    });
    assert_eq!(b.unwrap(), arr![4, 8, 12, 16]);
}

#[test]
fn test_try_from_fallible_iter() {
    use generic_array::typenum::U4;

    // success
    let ok: Result<GenericArray<i32, U4>, ()> =
        GenericArray::try_from_fallible_iter((0..4).map(Ok)).unwrap();
    assert_eq!(ok.unwrap(), arr![0, 1, 2, 3]);

    // length too short
    let short = GenericArray::<i32, U4>::try_from_fallible_iter((0..2).map(Ok::<_, ()>));
    assert!(short.is_err());

    // length too long
    let long = GenericArray::<i32, U4>::try_from_fallible_iter((0..6).map(Ok::<_, ()>));
    assert!(long.is_err());

    // inner error mid-way drops the already-initialized elements
    let counter = Cell::new(0);
    {
        let res: Result<Result<GenericArray<Tracked, U4>, &str>, _> =
            GenericArray::try_from_fallible_iter((0..4).map(|i| {
                if i == 2 {
                    Err("boom")
                } else {
                    Ok(Tracked(i, &counter))
                }
            }));
        assert_eq!(res.unwrap(), Err("boom"));
    }
    assert_eq!(counter.get(), 2);
}

#[test]
fn test_len_and_from_slice_panic_paths() {
    use generic_array::typenum::U4;
    assert_eq!(GenericArray::<i32, U4>::len(), 4);
}

#[test]
#[should_panic]
fn test_from_slice_wrong_length_panics() {
    use generic_array::typenum::U3;
    let _ = GenericArray::<i32, U3>::from_slice(&[1, 2]);
}

#[test]
fn test_chunks_from_slice_mut_zero_length() {
    let mut empty: [u8; 0] = [];
    let (chunks, rem) = GenericArray::<u8, U0>::chunks_from_slice_mut(&mut empty);
    assert!(chunks.is_empty());
    assert!(rem.is_empty());
}

#[test]
fn test_hash() {
    use core::hash::{Hash, Hasher};

    // A tiny additive hasher keeps this `no_std`-friendly and deterministic.
    struct SumHasher(u64);
    impl Hasher for SumHasher {
        fn finish(&self) -> u64 {
            self.0
        }
        fn write(&mut self, bytes: &[u8]) {
            for b in bytes {
                self.0 = self.0.wrapping_add(*b as u64);
            }
        }
    }

    fn hash_of(a: &GenericArray<u8, generic_array::typenum::U4>) -> u64 {
        let mut h = SumHasher(0);
        a.hash(&mut h);
        h.finish()
    }

    let a = arr![1u8, 2, 3, 4];
    let b = arr![1u8, 2, 3, 4];
    let c = arr![9u8, 2, 3, 4];

    assert_eq!(hash_of(&a), hash_of(&b));
    assert_ne!(hash_of(&a), hash_of(&c));
}

#[test]
fn test_functional_defaults_via_reference() {
    use generic_array::typenum::U4;

    let a = arr![2, 4, 6, 8];

    // try_map / try_fold trait defaults (owned GenericArray overrides these,
    // so they're only reached through the &S / Box impls)
    let mapped: Result<GenericArray<i32, U4>, ()> = (&a).try_map(|x| Ok(x * 2));
    assert_eq!(mapped.unwrap(), arr![4, 8, 12, 16]);

    let folded: Result<i32, ()> = (&a).try_fold(0, |acc, x| Ok(acc + x));
    assert_eq!(folded.unwrap(), 20);

    let err: Result<i32, &str> = (&a).try_fold(0, |_, _| Err("nope"));
    assert_eq!(err, Err("nope"));
}

#[test]
fn test_reference_zip_drop_path() {
    use generic_array::typenum::U4;

    // `(&lhs).zip(rhs, ..)` uses the &S `zip` default -> `rhs.inverted_zip2(&lhs, ..)`,
    // exercising the owned `inverted_zip2` Drop branch (owned `.zip` uses `inverted_zip`).
    let lhs = arr![1, 2, 3, 4];
    let counter = Cell::new(0);
    {
        let rhs: GenericArray<Tracked, U4> =
            GenericArray::generate(|i| Tracked(i as i32, &counter));
        let summed: GenericArray<i32, U4> = (&lhs).zip(rhs, |l, r| l + r.0);
        assert_eq!(summed, arr![1, 3, 5, 7]);
    }
    assert_eq!(counter.get(), 4);
}

#[test]
fn test_reference_generate() {
    use generic_array::sequence::{FallibleGenericSequence, GenericSequence};
    use generic_array::typenum::U4;

    // GenericSequence::generate for &S and &mut S forward to S::generate.
    let g = <&GenericArray<i32, U4> as GenericSequence<i32>>::generate(|i| i as i32);
    assert_eq!(g, arr![0, 1, 2, 3]);

    let gm = <&mut GenericArray<i32, U4> as GenericSequence<i32>>::generate(|i| i as i32 + 1);
    assert_eq!(gm, arr![1, 2, 3, 4]);

    // FallibleGenericSequence::try_generate for &S and &mut S.
    let tg: Result<Result<GenericArray<i32, U4>, ()>, _> =
        <&GenericArray<i32, U4> as FallibleGenericSequence<i32>>::try_generate(|i| Ok(i as i32));
    assert_eq!(tg.unwrap().unwrap(), arr![0, 1, 2, 3]);

    let tgm: Result<Result<GenericArray<i32, U4>, ()>, _> =
        <&mut GenericArray<i32, U4> as FallibleGenericSequence<i32>>::try_generate(|i| {
            Ok(i as i32)
        });
    assert_eq!(tgm.unwrap().unwrap(), arr![0, 1, 2, 3]);
}

#[test]
fn test_try_from_fallible_iter_length_after_fill() {
    use generic_array::typenum::U4;

    // A `filter` iter has a loose size_hint, so it passes the pre-checks and the
    // post-fill length check (`iter.next().is_some()`) catches the overflow instead.
    let loose = (0..10).filter(|x| *x < 6).map(Ok::<_, ()>);
    let r = GenericArray::<i32, U4>::try_from_fallible_iter(loose);
    assert!(r.is_err());
}

#[test]
#[should_panic]
fn test_from_mut_slice_wrong_length_panics() {
    use generic_array::typenum::U3;
    let mut data = [1, 2];
    let _ = GenericArray::<i32, U3>::from_mut_slice(&mut data);
}

#[test]
#[should_panic]
fn test_chunks_from_slice_mut_zero_length_nonempty_panics() {
    let mut data = [1u8, 2, 3];
    let _ = GenericArray::<u8, U0>::chunks_from_slice_mut(&mut data);
}

#[test]
#[should_panic]
fn test_remove_out_of_bounds_panics() {
    let a = arr![1, 2, 3, 4];
    let _ = a.remove(4);
}

#[test]
#[should_panic]
fn test_swap_remove_out_of_bounds_panics() {
    let a = arr![1, 2, 3, 4];
    let _ = a.swap_remove(10);
}

#[test]
#[should_panic]
fn test_from_fallible_iter_length_fail_panics() {
    use generic_array::typenum::U4;

    // `FromFallibleIterator::from_fallible_iter` panics (not `Err`) on a length
    // mismatch with no inner error, mirroring `FromIterator`'s contract.
    let _: Result<GenericArray<i32, U4>, ()> =
        FromFallibleIterator::from_fallible_iter((0..2).map(Ok::<i32, ()>));
}

#[cfg(feature = "internals")]
mod internals {
    use super::Cell;
    use core::mem::MaybeUninit;
    use generic_array::arr;
    use generic_array::internals::{ArrayBuilder, ArrayConsumer, IntrusiveArrayBuilder};
    use generic_array::sequence::GenericSequence;
    use generic_array::typenum::U4;
    use generic_array::GenericArray;

    #[test]
    fn test_intrusive_builder_finish() {
        // Exercises the `finish` (mem::forget) path, distinct from `finish_and_assume_init`.
        let mut array = MaybeUninit::<GenericArray<i32, U4>>::uninit();
        let result;
        {
            let mut builder = IntrusiveArrayBuilder::new_alt(&mut array);
            unsafe {
                let (dst_iter, position) = builder.iter_position();
                for (i, dst) in dst_iter.enumerate() {
                    dst.write(i as i32);
                    *position += 1;
                }
                builder.finish();
            }
        }
        result = unsafe { array.assume_init() };
        assert_eq!(result, arr![0, 1, 2, 3]);
    }

    #[test]
    #[allow(deprecated)]
    fn test_array_assume_init_deprecated() {
        let mut uninit = GenericArray::<i32, U4>::uninit();
        for (i, slot) in uninit.iter_mut().enumerate() {
            slot.write(i as i32);
        }
        let array = unsafe { IntrusiveArrayBuilder::array_assume_init(uninit) };
        assert_eq!(array, arr![0, 1, 2, 3]);
    }

    #[test]
    fn test_array_builder() {
        let mut builder = ArrayBuilder::<i32, U4>::new();
        assert!(!builder.is_full());

        unsafe {
            let (dst_iter, position) = builder.iter_position();
            for (i, dst) in dst_iter.enumerate() {
                dst.write(i as i32 * 2);
                *position += 1;
            }
            assert!(builder.is_full());
            let array = builder.assume_init();
            assert_eq!(array, arr![0, 2, 4, 6]);
        }
    }

    #[test]
    fn test_array_builder_extend() {
        let mut builder = ArrayBuilder::<i32, U4>::new();
        unsafe {
            builder.extend([10, 20, 30, 40].into_iter());
            assert_eq!(builder.assume_init(), arr![10, 20, 30, 40]);
        }
    }

    #[test]
    fn test_array_builder_drops_partial() {
        // Drop the builder without filling it: the initialized prefix must drop.
        let counter = Cell::new(0u32);
        {
            let mut builder = ArrayBuilder::<super::Tracked, U4>::new();
            unsafe {
                let (dst_iter, position) = builder.iter_position();
                for (i, dst) in dst_iter.enumerate().take(2) {
                    dst.write(super::Tracked(i as i32, &counter));
                    *position += 1;
                }
            }
            // builder dropped here with only 2 of 4 initialized
        }
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_array_consumer() {
        let counter = Cell::new(0u32);
        {
            let array: GenericArray<super::Tracked, U4> =
                GenericArray::generate(|i| super::Tracked(i as i32, &counter));
            let mut consumer = ArrayConsumer::new(array);
            unsafe {
                let (iter, position) = consumer.iter_position();
                // consume only the first two; the rest drop with the consumer
                for src in iter.take(2) {
                    let _value = core::ptr::read(src);
                    *position += 1;
                }
            }
            // consumer dropped: 2 consumed values dropped here + 2 leftovers dropped by Drop impl
        }
        assert_eq!(counter.get(), 4);
    }
}
