use std::env;
use tectonic_geturl::{DefaultBackend, GetUrlBackend};
use tectonic_status_base::plain::PlainStatusBackend;

fn main() {
    if let Some(url) = env::args().nth(1) {
        let mut backend = DefaultBackend::default();
        let mut status = PlainStatusBackend::default();

        let response = backend.get_url(&url, &mut status).unwrap();
        println!("{}", response.status());
    } else {
        println!("Usage: geturl <url>");
    }
}
