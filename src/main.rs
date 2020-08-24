mod titan;
use libtitan::StatusCode;
use titan::Titan;

fn main() {
    let mut server: Titan = Titan::new();
    server.get(
        "/rust",
        Box::new(|res| {
            res.set_body("# Hello from Rust!");
            res.set_meta("text/gemini");
            res.set_status(StatusCode::Code20)
        }),
    );
    titan::start(&server);
}
