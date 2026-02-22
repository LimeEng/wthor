use quickcheck_macros::quickcheck;

#[quickcheck]
#[allow(clippy::needless_pass_by_value)]
fn no_panic_game_archive(data: Vec<u8>) {
    // Just test that parsing doesn't panic
    let _result = wthor::parse(&data);
}

#[quickcheck]
#[allow(clippy::needless_pass_by_value)]
fn no_panic_database(data: Vec<u8>) {
    // Test that database loading doesn't panic
    let _result = wthor::Database::new()
        .with_players(&data)
        .and_then(|db| db.with_tournaments(&data));
}
