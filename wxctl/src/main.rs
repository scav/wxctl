#![warn(clippy::all, clippy::pedantic)]
use clap::{Parser, crate_version};

use wx::{
    client::{
        location::LocationApi,
        open_meteo::OpenMeteo,
        weather::{Backend, WeatherApi},
        yr::Yr,
    },
    errors::WxError,
};

#[derive(Parser)]
#[command(name = "wxctl")]
#[command(version = crate_version!())]
#[command(about = "Get weather based on location and country.\nIf unable to lookup a value, output will be empty.", long_about = None)]
struct Cli {
    #[arg(short, long = "name")]
    name: Option<String>,
    #[arg(short, long = "country")]
    country: Option<String>,
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
    /// Select backend yr|open-meteo.
    #[arg(short, long = "backend", default_value = "yr")]
    backend: Option<String>,
}

fn run(c: &Cli) -> Result<(), WxError> {
    let name = c.name.as_ref().ok_or(WxError::MissingName)?;
    let country = c.country.as_ref().ok_or(WxError::MissingCountry)?;

    let client = match c.backend.as_ref().map(String::as_str) {
        Some("yr") => Ok(Backend::Yr(Yr {})),
        Some("open-meteo") => Ok(Backend::OpenMeteo(OpenMeteo {})),
        Some(&_) => Err(WxError::InvalidBackend(c.backend.clone().unwrap())),
        None => Ok(Backend::Yr(Yr {})),
    }?;

    let location = client.get_lat_long(name, country)?;

    if c.debug {
        eprintln!("{location:?}");
    }

    let weather = client.get_current_weather(location.latitude, location.longitude)?;

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
