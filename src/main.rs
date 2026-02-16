#![warn(clippy::all, clippy::pedantic)]
mod client;
mod errors;
use clap::Parser;

use crate::errors::WxError;

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(version = "1.0")]
#[command(about = "Get weather based on location and country.\nIf unable to lookup a value, output will be empty.", long_about = None)]
struct Cli {
    #[arg(short, long = "name")]
    name: Option<String>,
    #[arg(short, long = "country")]
    country: Option<String>,
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

fn run(c: &Cli) -> Result<(), WxError> {
    let name = c.name.as_ref().ok_or(WxError::MissingName)?;
    let country = c.country.as_ref().ok_or(WxError::MissingCountry)?;

    let location = client::location::get_lat_long(name, country)?;

    if c.debug {
        eprintln!("{location:?}");
    }

    let weather = client::weather::get_current_weather(location.latitude, location.longitude)?;

    println!(
        "{}{}🌡 {}{}  ({}/{})",
        weather.current.temperature,
        weather.current_units.temperature,
        weather.current.humiditiy,
        weather.current_units.humiditiy,
        location.name,
        location.country_code
    );

    Ok(())
}

fn main() {
    let c = Cli::parse();

    if let Err(e) = run(&c) {
        if c.debug {
            eprintln!("{e:?}");
        }
        println!("{e}");
    }
}
