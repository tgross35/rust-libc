mod t1_stuff {
    include!(concat!(env!("OUT_DIR"), "/t1gen.rs"));
}

#[test]
fn t1() {
    let f = tempfile::tempfile().unwrap();
    t1_stuff::run(Some(Box::new(f)));
}
