use generic_array::{
    typenum::{self, Unsigned},
    GenericArray,
};

type Large = typenum::operator_aliases::Shleft<typenum::U1, typenum::U30>;
type LargeArray = GenericArray<(), Large>;

// run with `cargo miri test --test miri_stress`
#[test]
fn stress_miri() {
    let _ = LargeArray::from_array([(); Large::USIZE]); // This is a noop because `LargeArray` is a unit-like type
}
