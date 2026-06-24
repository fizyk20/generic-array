use generic_array::arr;
use generic_array::sequence::FromFallibleIterator as _;

use std::cell::Cell;
use std::ops::Drop;

use generic_array::typenum::consts::U5;
use generic_array::GenericArray;

#[test]
fn test_from_iterator() {
    struct BadExact(usize);

    impl Iterator for BadExact {
        type Item = usize;
        fn next(&mut self) -> Option<usize> {
            if self.0 == 1 {
                return None;
            }
            self.0 -= 1;
            Some(self.0)
        }
    }
    impl ExactSizeIterator for BadExact {
        fn len(&self) -> usize {
            self.0
        }
    }
    assert!(GenericArray::<usize, U5>::try_from_iter(BadExact(5)).is_err());
}

#[test]
fn test_into_iter_as_slice() {
    let array = arr!['a', 'b', 'c'];
    let mut into_iter = array.into_iter();
    assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
    let _ = into_iter.next().unwrap();
    assert_eq!(into_iter.as_slice(), &['b', 'c']);
    let _ = into_iter.next().unwrap();
    let _ = into_iter.next().unwrap();
    // Explicit type annotation needed because `rend` (pulled in by the `rkyv` feature) adds
    // additional `PartialEq` impls for `char`, leaving `&[]`'s element type ambiguous.
    assert_eq!(into_iter.as_slice(), &[] as &[char]);
}

#[test]
fn test_into_iter_as_mut_slice() {
    let array = arr!['a', 'b', 'c'];
    let mut into_iter = array.into_iter();
    assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
    into_iter.as_mut_slice()[0] = 'x';
    into_iter.as_mut_slice()[1] = 'y';
    assert_eq!(into_iter.next().unwrap(), 'x');
    assert_eq!(into_iter.as_slice(), &['y', 'c']);
}

#[test]
fn test_into_iter_debug() {
    let array = arr!['a', 'b', 'c'];
    let into_iter = array.into_iter();
    let debug = format!("{:?}", into_iter);
    assert_eq!(debug, "GenericArrayIter(['a', 'b', 'c'])");
}

#[test]
fn test_into_iter_clone() {
    fn iter_equal<I: Iterator<Item = i32>>(it: I, slice: &[i32]) {
        let v: Vec<i32> = it.collect();
        assert_eq!(&v[..], slice);
    }
    let mut it = arr![1, 2, 3].into_iter();
    iter_equal(it.clone(), &[1, 2, 3]);
    assert_eq!(it.next(), Some(1));
    let mut it = it.rev();
    iter_equal(it.clone(), &[3, 2]);
    assert_eq!(it.next(), Some(3));
    iter_equal(it.clone(), &[2]);
    assert_eq!(it.next(), Some(2));
    iter_equal(it.clone(), &[]);
    assert_eq!(it.next(), None);
}

#[test]
fn test_into_iter_nth() {
    let v = arr![0, 1, 2, 3, 4];
    for i in 0..v.len() {
        assert_eq!(v.into_iter().nth(i).unwrap(), v[i]);
    }
    assert_eq!(v.into_iter().nth(v.len()), None);

    let mut iter = v.into_iter();
    assert_eq!(iter.nth(2).unwrap(), v[2]);
    assert_eq!(iter.nth(1).unwrap(), v[4]);
}

#[test]
fn test_into_iter_nth_back() {
    let v = arr![0, 1, 2, 3, 4];

    for i in 0..v.len() {
        assert_eq!(v.into_iter().nth_back(i).unwrap(), v[v.len() - i - 1]);
    }
    assert_eq!(v.into_iter().nth_back(v.len()), None);

    let mut iter = v.into_iter();
    assert_eq!(iter.nth_back(2).unwrap(), v[2]);
    assert_eq!(iter.nth_back(1).unwrap(), v[0]);
}

#[test]
fn test_into_iter_last() {
    let v = arr![0, 1, 2, 3, 4];
    assert_eq!(v.into_iter().last().unwrap(), 4);
    assert_eq!(arr![0].into_iter().last().unwrap(), 0);
}

#[test]
fn test_into_iter_count() {
    let v = arr![0, 1, 2, 3, 4];
    assert_eq!(v.into_iter().count(), 5);

    let mut iter2 = v.into_iter();
    iter2.next();
    iter2.next();
    assert_eq!(iter2.count(), 3);
}

#[test]
fn test_into_iter_flat_map() {
    assert!((0..5).flat_map(|i| arr![2 * i, 2 * i + 1]).eq(0..10));
}

#[test]
fn test_into_iter_fold() {
    assert_eq!(arr![1, 2, 3, 4].into_iter().fold(0, |sum, x| sum + x), 10);

    let mut iter = arr![0, 1, 2, 3, 4, 5].into_iter();

    iter.next();
    iter.next_back();

    assert_eq!(iter.clone().fold(0, |sum, x| sum + x), 10);

    assert_eq!(iter.rfold(0, |sum, x| sum + x), 10);
}

#[test]
fn test_into_iter_drops() {
    struct R<'a> {
        i: &'a Cell<usize>,
    }

    impl<'a> Drop for R<'a> {
        fn drop(&mut self) {
            self.i.set(self.i.get() + 1);
        }
    }

    fn r(i: &'_ Cell<usize>) -> R<'_> {
        R { i }
    }

    fn v(i: &'_ Cell<usize>) -> GenericArray<R<'_>, U5> {
        arr![r(i), r(i), r(i), r(i), r(i)]
    }

    let i = Cell::new(0);
    {
        v(&i).into_iter();
    }
    assert_eq!(i.get(), 5);

    let i = Cell::new(0);
    {
        let mut iter = v(&i).into_iter();
        let _x = iter.next();
        assert_eq!(i.get(), 0);
        assert_eq!(iter.count(), 4);
        assert_eq!(i.get(), 4);
    }
    assert_eq!(i.get(), 5);

    let i = Cell::new(0);
    {
        let mut iter = v(&i).into_iter();
        let _x = iter.nth(2);
        assert_eq!(i.get(), 2);
        let _y = iter.last();
        assert_eq!(i.get(), 3);
    }
    assert_eq!(i.get(), 5);

    let i = Cell::new(0);
    {
        let mut iter = v(&i).into_iter();
        let _x = iter.nth_back(2);
        assert_eq!(i.get(), 2);
        let _y = iter.last();
        assert_eq!(i.get(), 3);
    }
    assert_eq!(i.get(), 5);

    let i = Cell::new(0);
    for (index, _x) in v(&i).into_iter().enumerate() {
        assert_eq!(i.get(), index);
    }
    assert_eq!(i.get(), 5);

    let i = Cell::new(0);
    for (index, _x) in v(&i).into_iter().rev().enumerate() {
        assert_eq!(i.get(), index);
    }
    assert_eq!(i.get(), 5);
}

// Targeted reproduction attempt for the Gemini audit's "slice fabrication over
// partially-moved arrays" finding. The claim: every `get_unchecked(index..index_back)`
// in the iterator auto-derefs through the whole-array `slice::from_raw_parts` over
// `0..N`, and once leading/trailing elements have been moved out, forming that
// whole-array reference is claimed to be UB.
//
// To give that claim the strongest possible chance to fire under Miri, the element
// type is `Niche(NonZeroU32)`: a moved-out slot left as a zeroed bit pattern would be
// a *validity-invalid* `NonZeroU32`, not merely uninitialized - so a reference spanning
// it would be UB that Tree Borrows + validity checking must catch. A `Cell`-backed Drop
// counter additionally proves exactly-once drop accounting through each path.
//
// The helpers below exercise *every* slice-forming call site the report named
// (`as_slice`, `as_mut_slice`, `next`, `nth`, `next_back`, `nth_back`, `fold`, `rfold`,
// `Drop`, and `Clone`) while the backing array is in a partially-moved state.
//
// Run under both aliasing models to adjudicate:
//   cargo +nightly miri test --test iter partial_move
//   MIRIFLAGS="-Zmiri-tree-borrows" cargo +nightly miri test --test iter partial_move

use std::num::NonZeroU32;

struct Niche<'a>(NonZeroU32, &'a Cell<u32>);

impl Clone for Niche<'_> {
    fn clone(&self) -> Self {
        Niche(self.0, self.1)
    }
}

impl Drop for Niche<'_> {
    fn drop(&mut self) {
        // Touch the value so a moved-out/zeroed slot is observable as a validity bug,
        // and count the drop so callers can assert exactly-once semantics.
        assert!(self.0.get() != 0, "dropped a moved-out / invalid Niche");
        self.1.set(self.1.get() + 1);
    }
}

// Build a fresh 5-element iterator whose elements all count drops into `c`.
fn mk_iter(c: &Cell<u32>) -> generic_array::GenericArrayIter<Niche<'_>, U5> {
    GenericArray::<Niche, U5>::from_iter(
        (1..=5).map(|n| Niche(NonZeroU32::new(n).unwrap(), c)),
    )
    .into_iter()
}

#[test]
fn test_partial_move_as_slice_both_ends() {
    let c = Cell::new(0);
    {
        let mut iter = mk_iter(&c);
        let _front = iter.next().unwrap(); // index advances; slot 0 moved out
        let _back = iter.next_back().unwrap(); // index_back retreats; slot 4 moved out
        // as_slice / as_mut_slice now form the whole-array ref with slots 0 and 4 dead.
        assert_eq!(iter.as_slice().len(), 3);
        for n in iter.as_mut_slice() {
            assert!(n.0.get() != 0);
        }
        // Debug also routes through as_slice().
        let _ = format!("{:?} {:?}", iter.as_slice().len(), c.get());
        drop(iter); // Drop forms the remaining [1..4] slice and drop_in_place's it.
    }
    assert_eq!(c.get(), 5, "all 5 elements dropped exactly once");
}

#[test]
fn test_partial_move_drain_to_empty() {
    // Every slot moved out before Drop: Drop must form a zero-length slice, not touch
    // any of the (now invalid) backing memory.
    let c = Cell::new(0);
    {
        let mut iter = mk_iter(&c);
        while iter.next().is_some() {}
        assert_eq!(iter.as_slice().len(), 0);
        drop(iter);
    }
    assert_eq!(c.get(), 5);
}

#[test]
fn test_partial_move_nth_and_nth_back() {
    // nth() drop_in_place's the skipped prefix slice, then next() reads through the
    // whole-array deref; nth_back() does the mirror on the suffix.
    let c = Cell::new(0);
    {
        let mut iter = mk_iter(&c);
        let _ = iter.nth(1).unwrap(); // drops slots [0..1], returns slot 1
        let _ = iter.nth_back(1).unwrap(); // drops slots [4..5)->[3..4], returns slot 3
        assert_eq!(iter.as_slice().len(), 1); // only slot 2 remains live
        drop(iter);
    }
    assert_eq!(c.get(), 5);
}

#[test]
fn test_partial_move_fold_after_consume() {
    // fold() forms get_unchecked(index..index_back) and ptr::reads through it while
    // mutating the index, with both outer ends already moved out.
    let c = Cell::new(0);
    {
        let mut iter = mk_iter(&c);
        let _ = iter.next().unwrap();
        let _ = iter.next_back().unwrap();
        let sum = iter.fold(0u32, |acc, n| acc + n.0.get());
        assert_eq!(sum, 2 + 3 + 4);
    }
    assert_eq!(c.get(), 5);
}

#[test]
fn test_partial_move_rfold_after_consume() {
    let c = Cell::new(0);
    {
        let mut iter = mk_iter(&c);
        let _ = iter.next().unwrap();
        let _ = iter.next_back().unwrap();
        let sum = iter.rfold(0u32, |acc, n| acc + n.0.get());
        assert_eq!(sum, 2 + 3 + 4);
    }
    assert_eq!(c.get(), 5);
}

#[test]
fn test_partial_move_clone_after_consume() {
    // Clone is the spiciest path: it ptr::read's the *entire* partially-moved backing
    // array (bitwise) into a new iter, then writes clones into the live prefix via
    // as_mut_slice(). If forming a ref over moved-out slots were UB, this is where it
    // would bite hardest.
    let c = Cell::new(0);
    {
        let mut iter = mk_iter(&c);
        let _ = iter.next().unwrap(); // slot 0 dead
        let _ = iter.next_back().unwrap(); // slot 4 dead
        let cloned = iter.clone(); // bitwise-copies [_, 2, 3, 4, _], clones live 2,3,4
        assert_eq!(cloned.as_slice().len(), 3);
        // Iterator::map (lazy) over the cloned by-value iter, consuming the 3 clones.
        let s: u32 = cloned.map(|n| n.0.get()).sum();
        assert_eq!(s, 2 + 3 + 4);
        drop(iter);
    }
    // 5 originals + 3 clones = 8 drops.
    assert_eq!(c.get(), 8);
}

#[test]
fn test_from_failing_iter() {
    let res: Result<GenericArray<_, U5>, ()> = GenericArray::from_fallible_iter(
        (2..).map(|x| if x == 5 { Err(()) } else { Ok(x) }).take(5),
    );

    assert!(res.is_err());
}

/*
//TODO: Cover this
#[allow(dead_code)]
fn assert_covariance() {
    fn into_iter<'new>(i: GenericArrayIter<&'static str, U10>) -> GenericArrayIter<&'new str, U10> {
        i
    }
}
*/
