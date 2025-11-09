use quickcheck_macros::quickcheck;

#[quickcheck]
#[allow(clippy::needless_pass_by_value)]
fn no_panic_game_archive(data: Vec<u8>) {
    // Just test that building doesn't panic
    let _result = wthor::parse(&data).build();
}

#[quickcheck]
#[allow(clippy::needless_pass_by_value)]
fn no_panic_records(data: Vec<u8>) {
    // Test both players and tournaments together
    let _result = wthor::parse(&data)
        .players(&data)
        .tournaments(&data)
        .build();
}
