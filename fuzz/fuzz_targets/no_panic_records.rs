#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _result = wthor::Players::try_from(data);
    let _result = wthor::Tournaments::try_from(data);
});
