use std::io::{self, Write};
mod client;
use clap::Parser;

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

fn main() {
    color_eyre::install().unwrap();
    let c = Cli::parse();
    if let (Some(name), Some(country)) = (c.name, c.country) {
        let loc = client::location::get_lat_long(&name, &country).unwrap();
        if c.debug {
            let mut out = io::stderr();
            _ = writeln!(out, "{:?}", loc);
        }
        let weather = client::weather::get_current_weather(loc.latitude, loc.longitude).unwrap();

        let mut out = io::stdout();
        _ = writeln!(
            out,
            "{}{}🌡 ({})",
            weather.current.temperature,
            weather.current_units.temperature,
            uppercase_first(&name)
        );
    };
}

fn uppercase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
