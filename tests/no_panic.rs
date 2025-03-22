use quickcheck_macros::quickcheck;
use wthor::{game_archive::GameArchive, records::Records};

#[quickcheck]
#[allow(clippy::needless_pass_by_value)]
fn no_panic_game_archive(data: Vec<u8>) {
    let _result = wthor::parse::<GameArchive>(&data);
}

#[quickcheck]
#[allow(clippy::needless_pass_by_value)]
fn no_panic_records(data: Vec<u8>) {
    let _result = wthor::parse::<Records>(&data);
}
