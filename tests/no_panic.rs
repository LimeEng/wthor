use quickcheck_macros::quickcheck;

#[quickcheck]
#[allow(clippy::needless_pass_by_value)]
fn no_panic_games(data: Vec<u8>) {
    let _result = wthor::Games::try_from(data.as_slice());
}

#[quickcheck]
#[allow(clippy::needless_pass_by_value)]
fn no_panic_players(data: Vec<u8>) {
    let _result = wthor::Players::try_from(data.as_slice());
}

#[quickcheck]
#[allow(clippy::needless_pass_by_value)]
fn no_panic_tournaments(data: Vec<u8>) {
    let _result = wthor::Tournaments::try_from(data.as_slice());
}
