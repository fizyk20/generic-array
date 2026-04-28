#![cfg(feature = "alloc")]
#![no_std]
use core::cell::Cell;
use core::ops::Drop;
use core::sync::atomic::{AtomicU32, Ordering};
use generic_array::arr;
use generic_array::functional::*;
use generic_array::sequence::*;
use generic_array::typenum::{U0, U4};
use generic_array::GenericArray;

extern crate alloc;

#[test]
fn test_try_from_vec() {
    let a = alloc::vec![1, 2, 3, 4];
    let _ = GenericArray::<_, U4>::try_from(a).unwrap();
}

#[test]
fn test_alloc() {
    use alloc::{boxed::Box, vec::Vec};
    use generic_array::box_arr;

    let x: Box<[i32]> = arr![1, 2, 3, 4, 5].into();
    assert_eq!(x.len(), 5);
    let y: GenericArray<i32, typenum::U5> = x.clone().try_into().unwrap();
    assert_eq!(&x[..], &y[..]);

    let x: Vec<i32> = arr![1, 2, 3, 4, 5].into();
    assert_eq!(x.len(), 5);
    let y: GenericArray<i32, typenum::U5> = x.clone().try_into().unwrap();
    assert_eq!(&x[..], &y[..]);

    let x: Vec<i32> = Box::new(arr![1, 2, 3, 4, 5]).into_vec();
    assert_eq!(x.len(), 5);
    let y: Box<GenericArray<i32, typenum::U5>> = GenericArray::try_from_vec(x.clone()).unwrap();
    assert_eq!(&x[..], &y[..]);

    let z =
        Box::<GenericArray<_, typenum::U5>>::from_iter(y.into_iter() as alloc::vec::IntoIter<_>);

    let _: Box<_> = z.clone().zip(Box::new(arr![1, 2, 3, 4, 5]), |a, b| a + b);

    let _ = z.map(|x| x + 1);

    let _ = arr![1, 2, 3, 4].zip(*box_arr![1, 2, 3, 4], |a, b| a + b);

    let _ = box_arr!(1, 2, 3, 4, 5);

    let _: Box<GenericArray<u32, U0>> = GenericArray::default_boxed();

    #[cfg(not(miri))]
    {
        // 128-bit * 10^6 = 16MB, large enough to overflow the stack, but not this
        let _ = box_arr![1u128; typenum::Exp<typenum::U10, typenum::U6>];

        let _ = GenericArray::<i128, typenum::Exp<typenum::U10, typenum::U6>>::default_boxed();
    }
}

// Exercises Box<GenericArray>::try_generate through IntrusiveBoxedArrayBuilder:
// - the iter_position split borrow (ptr field + position field simultaneously)
// - drop_in_place on the initialized prefix on error
// - dealloc after a partial fill
#[test]
fn miri_box_try_generate_drop_on_error() {
    use alloc::boxed::Box;
    use core::cell::Cell;

    struct Tracked<'a>(&'a Cell<u32>);
    impl Drop for Tracked<'_> {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let counter = Cell::new(0u32);

    // error on first element: alloc happened but drop path must free it cleanly
    let r: Result<Result<Box<GenericArray<Tracked<'_>, U4>>, ()>, _> =
        <Box<GenericArray<Tracked<'_>, U4>>>::try_generate(|_| Err(()));
    assert!(r.unwrap().is_err());
    assert_eq!(counter.get(), 0);

    // error at index 3: three elements initialized, all three dropped, memory freed
    let r: Result<Result<Box<GenericArray<Tracked<'_>, U4>>, &str>, _> =
        <Box<GenericArray<Tracked<'_>, U4>>>::try_generate(|i| {
            if i == 3 {
                Err("stop")
            } else {
                Ok(Tracked(&counter))
            }
        });
    assert!(r.unwrap().is_err());
    assert_eq!(counter.get(), 3);
}

// Exercises Box<GenericArray> ZST allocation path (dangling NonNull pointer).
// Also exercises generate (not try_generate) to ensure the non-fallible heap path is clean.
#[test]
fn miri_box_zst_array() {
    use alloc::boxed::Box;

    let _: Box<GenericArray<u32, U0>> = <Box<GenericArray<u32, U0>>>::generate(|_| 0);

    let r: Result<Result<Box<GenericArray<u32, U0>>, ()>, _> =
        <Box<GenericArray<u32, U0>>>::try_generate(|_| Err(()));
    // generator never called for N=0
    assert!(r.unwrap().is_ok());
}

// Exercises the into_boxed_slice / try_from_boxed_slice raw pointer round-trips.
// Miri validates that pointer casts and provenance are correct.
#[test]
fn miri_boxed_slice_round_trip() {
    use alloc::boxed::Box;
    use generic_array::{arr, LengthError};

    let arr: Box<GenericArray<u32, U4>> = Box::new(arr![1, 2, 3, 4]);
    let slice: Box<[u32]> = arr.into_boxed_slice();
    assert_eq!(&*slice, &[1, 2, 3, 4]);

    let back: Box<GenericArray<u32, U4>> = GenericArray::try_from_boxed_slice(slice).unwrap();
    assert_eq!(&*back, &arr![1, 2, 3, 4]);

    // wrong length must return Err without freeing the box
    let wrong: Box<[u32]> = alloc::vec![1u32, 2, 3].into_boxed_slice();
    let err: Result<Box<GenericArray<u32, U4>>, LengthError> =
        GenericArray::try_from_boxed_slice(wrong);
    assert!(err.is_err());
}

#[test]
fn test_box_generate_zst_runs_closure_per_index() {
    use alloc::boxed::Box;

    let counter = Cell::new(0u32);
    let _: Box<GenericArray<(), U4>> = <Box<GenericArray<(), U4>>>::generate(|_| {
        counter.set(counter.get() + 1);
    });
    assert_eq!(counter.get(), 4);
}

#[test]
fn test_box_try_generate_zst_runs_closure_per_index() {
    use alloc::boxed::Box;

    let counter = Cell::new(0u32);
    let r: Result<Result<Box<GenericArray<(), U4>>, ()>, _> =
        <Box<GenericArray<(), U4>>>::try_generate(|_| {
            counter.set(counter.get() + 1);
            Ok(())
        });
    assert!(r.unwrap().is_ok());
    assert_eq!(counter.get(), 4);
}

#[test]
fn test_box_try_generate_zst_drops_initialized_on_error() {
    use alloc::boxed::Box;

    static ZST_TRY_GEN_ERR_DROPS: AtomicU32 = AtomicU32::new(0);

    struct ZstDrop;
    impl Drop for ZstDrop {
        fn drop(&mut self) {
            ZST_TRY_GEN_ERR_DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }
    assert_eq!(core::mem::size_of::<ZstDrop>(), 0);

    let r: Result<Result<Box<GenericArray<ZstDrop, U4>>, &str>, _> =
        <Box<GenericArray<ZstDrop, U4>>>::try_generate(|i| {
            if i == 2 {
                Err("stop")
            } else {
                Ok(ZstDrop)
            }
        });
    assert!(r.unwrap().is_err());
    assert_eq!(ZST_TRY_GEN_ERR_DROPS.load(Ordering::Relaxed), 2);
}

// True ZST (size == 0, has Drop): exercises the dangling-pointer branch in
// IntrusiveBoxedArrayBuilder. Static counters are needed because a ZST cannot
// hold a reference.
static TRUE_ZST_GEN_DROPS: AtomicU32 = AtomicU32::new(0);

#[test]
fn test_true_zst_generate_invokes_and_drops() {
    use alloc::boxed::Box;

    struct TrueZst;
    impl Drop for TrueZst {
        fn drop(&mut self) {
            TRUE_ZST_GEN_DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }
    assert_eq!(core::mem::size_of::<TrueZst>(), 0);

    let before = TRUE_ZST_GEN_DROPS.load(Ordering::Relaxed);
    let b: Box<GenericArray<TrueZst, U4>> = <Box<GenericArray<TrueZst, U4>>>::generate(|_| TrueZst);
    assert_eq!(TRUE_ZST_GEN_DROPS.load(Ordering::Relaxed) - before, 0);
    drop(b);
    assert_eq!(TRUE_ZST_GEN_DROPS.load(Ordering::Relaxed) - before, 4);
}

#[test]
fn test_zst_try_generate_drops_initialized_on_error() {
    use alloc::boxed::Box;

    static TRUE_ZST_TRY_DROPS: AtomicU32 = AtomicU32::new(0);

    struct TrueZst;
    impl Drop for TrueZst {
        fn drop(&mut self) {
            TRUE_ZST_TRY_DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }
    assert_eq!(core::mem::size_of::<TrueZst>(), 0);

    let before = TRUE_ZST_TRY_DROPS.load(Ordering::Relaxed);
    let r: Result<Result<Box<GenericArray<TrueZst, U4>>, &str>, _> =
        <Box<GenericArray<TrueZst, U4>>>::try_generate(|i| {
            if i == 2 {
                Err("stop")
            } else {
                Ok(TrueZst)
            }
        });
    assert!(r.unwrap().is_err());
    assert_eq!(TRUE_ZST_TRY_DROPS.load(Ordering::Relaxed) - before, 2);
}

#[test]
fn test_box_try_generate_uninhabited_returns_user_error() {
    use alloc::boxed::Box;
    use core::convert::Infallible;
    use generic_array::typenum::U2;

    let calls = Cell::new(0u32);
    let r: Result<Result<Box<GenericArray<Infallible, U2>>, &str>, _> =
        <Box<GenericArray<Infallible, U2>>>::try_generate(|_| {
            calls.set(calls.get() + 1);
            Err("never produced")
        });
    assert_eq!(r.unwrap(), Err("never produced"));
    assert_eq!(calls.get(), 1);
}

#[test]
fn test_box_generate_uninhabited_invokes_closure() {
    extern crate std;
    use alloc::boxed::Box;
    use core::convert::Infallible;
    use core::panic::AssertUnwindSafe;
    use generic_array::typenum::U2;

    let observed = std::panic::catch_unwind(AssertUnwindSafe(|| {
        let _: Box<GenericArray<Infallible, U2>> =
            <Box<GenericArray<Infallible, U2>>>::generate(|_| panic!("closure ran"));
    }));
    assert!(observed.is_err());
}

#[test]
fn test_box_try_generate_success() {
    use alloc::boxed::Box;
    use generic_array::sequence::FallibleGenericSequence as _;

    let a: Box<GenericArray<i32, U4>> =
        <Box<GenericArray<i32, U4>>>::try_generate(|i| Ok::<_, ()>(i as i32 + 1))
            .unwrap()
            .unwrap();
    assert_eq!(&*a, &arr![1, 2, 3, 4]);
}

#[test]
fn test_box_try_generate_error() {
    use alloc::boxed::Box;
    use generic_array::sequence::FallibleGenericSequence as _;

    let result: Result<Result<Box<GenericArray<i32, U4>>, &str>, _> =
        <Box<GenericArray<i32, U4>>>::try_generate(
            |i| {
                if i == 1 {
                    Err("stop")
                } else {
                    Ok(i as i32)
                }
            },
        );
    assert_eq!(result.unwrap(), Err("stop"));
}

#[test]
fn test_box_try_generate_drops_on_error() {
    use alloc::boxed::Box;
    use generic_array::sequence::FallibleGenericSequence as _;

    #[derive(Clone)]
    struct Tracked<'a>(&'a Cell<u32>);

    impl<'a> Drop for Tracked<'a> {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let counter = Cell::new(0u32);
    let result: Result<Result<Box<GenericArray<Tracked<'_>, U4>>, &str>, _> =
        <Box<GenericArray<Tracked<'_>, U4>>>::try_generate(|i| {
            if i == 2 {
                Err("stop")
            } else {
                Ok(Tracked(&counter))
            }
        });
    assert!(result.unwrap().is_err());
    assert_eq!(
        counter.get(),
        2,
        "exactly 2 initialized elements should be dropped"
    );
}

#[test]
fn test_box_try_generate_zero_length() {
    use alloc::boxed::Box;
    use generic_array::sequence::FallibleGenericSequence as _;
    use generic_array::typenum::U0;

    let _: Box<GenericArray<i32, U0>> =
        <Box<GenericArray<i32, U0>>>::try_generate(|_| Ok::<_, ()>(0))
            .unwrap()
            .unwrap();
}

#[test]
fn test_box_from_fallible_iter() {
    use alloc::{boxed::Box, vec};
    use generic_array::sequence::FromFallibleIterator as _;

    // success
    let a: Box<GenericArray<i32, U4>> =
        Box::from_fallible_iter(vec![Ok::<_, ()>(1), Ok(2), Ok(3), Ok(4)]).unwrap();
    assert_eq!(&*a, &arr![1, 2, 3, 4]);

    // error in iterator
    let e: Result<Box<GenericArray<i32, U4>>, &str> =
        Box::from_fallible_iter(vec![Ok(1), Ok(2), Err("bad"), Ok(4)]);
    assert_eq!(e, Err("bad"));
}

#[test]
#[should_panic]
fn test_box_from_fallible_iter_too_short() {
    use alloc::{boxed::Box, vec};
    use generic_array::sequence::FromFallibleIterator as _;

    let _: Box<GenericArray<i32, U4>> =
        Box::from_fallible_iter(vec![Ok::<_, ()>(1), Ok(2), Ok(3)]).unwrap();
}

#[test]
#[should_panic]
fn test_box_from_fallible_iter_too_long() {
    use alloc::{boxed::Box, vec};
    use generic_array::sequence::FromFallibleIterator as _;

    let _: Box<GenericArray<i32, U4>> =
        Box::from_fallible_iter(vec![Ok::<_, ()>(1), Ok(2), Ok(3), Ok(4), Ok(5)]).unwrap();
}

#[test]
fn test_alloc_error_display() {
    use generic_array::AllocError;

    assert_eq!(alloc::format!("{}", AllocError), "memory allocation failed");
}

#[test]
fn test_length_error_display() {
    use generic_array::LengthError;

    let msg = alloc::format!("{}", LengthError);
    assert!(msg.contains("LengthError"));
}
