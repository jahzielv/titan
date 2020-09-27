mod titan;
use titan::Titan;

fn main() {
    let mut server: Titan = Titan::new();
    server.read_config_file();

    titan::start(&server);
}
