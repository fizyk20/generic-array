use generic_array as gen_arr;

#[test]
fn test_different_crate_name() {
    use gen_arr::arr;
    use gen_arr::typenum;

    let _: gen_arr::GenericArray<u32, typenum::U4> = arr![0, 1, 2, 3];
    let _: gen_arr::GenericArray<u32, typenum::U0> = arr![];
}

#[test]
fn test_crate_usage() {
    let _: gen_arr::GenericArray<u32, typenum::U0> = gen_arr::arr![];
}
