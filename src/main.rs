use color_eyre::Report;
use config::Config;

mod config;
mod utils;

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let config_path = "sample.kdl";
    let config_text = r#"
    environment "development"

    backend {
        listen "127.0.0.1:5000"
    }

    frontend {
        listen "127.0.0.1:8080"
        backend "127.0.0.1:5000"
    }
    "#;

    let config = Config::parse(config_path, config_text)?;
    println!("here it is: {config:#?}");

    Ok(())
}
