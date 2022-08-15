use quickcheck_macros::quickcheck;

#[quickcheck]
fn no_panic(data: Vec<u8>) {
    let _result = wthor::parse(&data);
}
