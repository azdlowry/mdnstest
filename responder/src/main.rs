extern crate env_logger;
extern crate libmdns;

fn main() {
    env_logger::init();

    let responder = libmdns::Responder::new().unwrap();
    let _svc = responder.register(
        "_presciense._poltergeist._tcp".to_owned(),
        "DEADBEEFDEADBEEF".to_owned(),
        8889,
        &["device=DEADBEEFDEADBEEF"]);

    loop {
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}
