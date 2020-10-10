mod titan;
use failure::Error;
use titan::Titan;

fn main() -> Result<(), Error> {
    let mut server: Titan = Titan::new();
    server.read_config_file()?;

    Ok(titan::start(&server)?)
}
