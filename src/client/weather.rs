use crate::errors::WxError;
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

pub fn get_current_weather(lat: f32, long: f32) -> Result<Weather, WxError> {
    let weather: Weather = ureq::get(
        format!("https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={long}&current=temperature_2m,relative_humidity_2m,apparent_temperature,is_day,precipitation,weather_code"))
        .call()?
        .body_mut()
        .read_json()?;

    Ok(weather)
}

#[cfg(test)]
mod test {}
