mod titan;
use libtitan::StatusCode;
use std::fs;
use titan::Titan;
use toml::Value;

fn main() {
    let mut server: Titan = Titan::new();

    let config_string = fs::read_to_string("Titan.toml").unwrap();
    let map = config_string.parse::<Value>().unwrap();
    if let Value::Table(t) = &map["routes"] {
        let x = t.clone();
        for (key, value) in x {
            server.get(
                &key,
                Box::new(move |res| {
                    res.set_body(&fs::read_to_string(value.as_str().unwrap()).unwrap());
                    res.set_meta("text/gemini");
                    res.set_status(StatusCode::Success);
                }),
            );
        }
    }

    titan::start(&server);
}
