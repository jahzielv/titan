mod titan;
use libtitan::StatusCode;
use std::fs;
use titan::Titan;

fn main() {
    let mut server: Titan = Titan::new();
    server.get(
        "/rust",
        Box::new(|res| {
            res.set_body("# Hello from Rust!");
            res.set_meta("text/gemini");
            res.set_status(StatusCode::Success)
        }),
    );
    server.get(
        "/about",
        Box::new(|res| {
            let body = fs::read_to_string("about.gemini").unwrap();
            res.set_body(&body);
            res.set_meta("text/gemini");
            res.set_status(StatusCode::Success);
        }),
    );

    server.get(
        "/error",
        Box::new(|res| {
            res.set_body("");
            res.set_meta("1234");
            res.set_status(StatusCode::SlowDown);
        }),
    );
    titan::start(&server);
}
