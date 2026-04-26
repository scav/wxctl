use crate::{
    client::{location::LocationApi, open_meteo::OpenMeteo, yr::Yr},
    errors::WxError,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub current_units: WeatherUnits,
    pub current: WeatherCurrent,
}
#[derive(Deserialize, Debug)]
pub struct WeatherCurrent {
    #[serde(rename = "temperature_2m")]
    pub temperature: f32,
    #[serde(rename = "relative_humidity_2m")]
    pub humiditiy: i32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherUnits {
    #[serde(rename = "temperature_2m")]
    pub temperature: String,
    #[serde(rename = "relative_humidity_2m")]
    pub humiditiy: String,
}

#[derive(Debug)]
pub enum Backend {
    Yr(Yr),
    OpenMeteo(OpenMeteo),
}

impl WeatherApi for Backend {
    fn get_current_weather(&self, lat: f32, long: f32) -> Result<Weather, WxError> {
        match self {
            Backend::Yr(y) => y.get_current_weather(lat, long),
            Backend::OpenMeteo(o) => o.get_current_weather(lat, long),
        }
    }
}
impl LocationApi for Backend {
    fn get_lat_long(
        &self,
        name: &str,
        country: &str,
    ) -> Result<super::location::Location, WxError> {
        match self {
            Backend::Yr(y) => y.get_lat_long(name, country),
            Backend::OpenMeteo(o) => o.get_lat_long(name, country),
        }
    }
}

pub trait WeatherApi {
    fn get_current_weather(&self, lat: f32, long: f32) -> Result<Weather, WxError>;
    fn temperature_unit(&self, unit: String) -> String {
        let c = "°C";
        let f = "°F";
        if unit != "celsius" {
            return f.to_string();
        }
        c.to_string()
    }
}

#[cfg(test)]
mod test {}
