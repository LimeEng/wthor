#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _result = wthor::Database::new()
        .with_players(data)
        .and_then(|db| db.with_tournaments(data));
});
