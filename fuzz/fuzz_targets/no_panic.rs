#![no_main]
use libfuzzer_sys::fuzz_target;
use wthor;

fuzz_target!(|data: &[u8]| {
    let _ = wthor::parse(data);
});
