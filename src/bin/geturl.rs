use std::env;
use tectonic_geturl::{plain::PlainStatusBackend, DefaultBackend, GetUrlBackend};

fn main() {
    if let Some(url) = env::args().nth(1) {
        let mut backend = DefaultBackend::default();
        let mut status = PlainStatusBackend::default();

        let mut response = backend.get_url(&url, &mut status).unwrap();

        #[cfg(feature = "curl")]
        {
            use std::io::Read;
            let mut buf = Vec::new();
            response.read_to_end(&mut buf).unwrap();
            println!("{} bytes", buf.len());
            // try to convert buf to a string
            let s = String::from_utf8_lossy(&buf);
            println!("{}", s);
        }
        #[cfg(not(feature = "curl"))]
        {
            println!("response: {:?}", response);
            println!("status: {}", response.status());
        }
    } else {
        println!("Usage: geturl <url>");
    }
}
