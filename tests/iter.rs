use generic_array::arr;

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
    assert_eq!(into_iter.as_slice(), &[]);
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

/*
//TODO: Cover this
#[allow(dead_code)]
fn assert_covariance() {
    fn into_iter<'new>(i: GenericArrayIter<&'static str, U10>) -> GenericArrayIter<&'new str, U10> {
        i
    }
}
*/
