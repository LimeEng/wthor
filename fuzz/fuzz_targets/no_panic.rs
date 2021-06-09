#![no_main]
use libfuzzer_sys::fuzz_target;
use wthor::parse::parse;

fuzz_target!(|data: &[u8]| {
    let _ = parse(data);
});
