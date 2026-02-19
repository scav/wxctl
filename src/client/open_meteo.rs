use crate::{
    client::{
        location::{Location, LocationApi, LocationResult},
        weather::{Weather, WeatherApi},
    },
    errors::WxError,
};

#[derive(Debug)]
pub struct OpenMeteo {}

impl WeatherApi for OpenMeteo {
    fn get_current_weather(
        &self,
        lat: f32,
        long: f32,
    ) -> Result<super::weather::Weather, crate::errors::WxError> {
        let weather: Weather = ureq::get(
        format!("https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={long}&current=temperature_2m,relative_humidity_2m,apparent_temperature,is_day,precipitation,weather_code"))
        .call()?
        .body_mut()
        .read_json()?;

        Ok(weather)
    }
}

impl LocationApi for OpenMeteo {
    fn get_lat_long(&self, name: &str, country: &str) -> Result<Location, WxError> {
        let c: LocationResult = ureq::get(format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={name}%2C+{country}",
        ))
        .call()?
        .body_mut()
        .read_json()
        .map_err(WxError::Response)?;

        // Just pick the first location
        Ok(c.results
            .first()
            .ok_or_else(|| WxError::EmptyResult)?
            .clone())
    }
}
