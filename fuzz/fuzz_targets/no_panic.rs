#![no_main]
use libfuzzer_sys::fuzz_target;

use wthor::{game_archive::GameArchive, records::Records};

fuzz_target!(|data: &[u8]| {
    let _result = wthor::parse::<GameArchive>(data);
});

fuzz_target!(|data: &[u8]| {
    let _result = wthor::parse::<Records>(data);
});
