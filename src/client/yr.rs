use serde::Deserialize;

use crate::{
    client::{
        location::{Location, LocationApi},
        weather::{Weather, WeatherApi, WeatherCurrent, WeatherUnits},
    },
    errors::WxError,
};

#[derive(Deserialize, Debug, Clone)]
struct OsmLocation {
    name: String,
    lat: String,
    lon: String,
    address: OsmAddress,
}

#[derive(Deserialize, Debug, Clone)]
struct OsmAddress {
    country_code: String,
}
#[derive(Debug, Deserialize)]
pub struct YrResponse {
    pub properties: Properties,
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    pub meta: YrMetaWithUnits,
    pub timeseries: Vec<TimeSeries>,
}

#[derive(Debug, Deserialize)]
pub struct YrMetaWithUnits {
    pub units: Units,
}

#[derive(Debug, Deserialize)]
pub struct Units {
    pub air_temperature: String,
    pub relative_humidity: String,
}

#[derive(Debug, Deserialize)]
pub struct TimeSeries {
    pub data: TimeData,
}

#[derive(Debug, Deserialize)]
pub struct TimeData {
    pub instant: InstantData,
}

#[derive(Debug, Deserialize)]
pub struct InstantData {
    pub details: YrWeatherDetails,
}

#[derive(Debug, Deserialize)]
pub struct YrWeatherDetails {
    pub air_temperature: f32,
    pub relative_humidity: f32,
}

#[derive(Debug)]
pub struct Yr {}

impl WeatherApi for Yr {
    fn get_current_weather(
        &self,
        lat: f32,
        long: f32,
    ) -> Result<super::weather::Weather, crate::errors::WxError> {
        let result: YrResponse = ureq::get(&format!(
            "https://api.met.no/weatherapi/locationforecast/2.0/compact?lat={}&lon={}",
            lat, long
        ))
        .call()?
        .body_mut()
        .read_json()?;

        let weather = result
            .properties
            .timeseries
            .get(1) // Apparently the first item is an hour in the past?
            .ok_or_else(|| WxError::EmptyResult)?;

        Ok(Weather {
            current_units: WeatherUnits {
                temperature: self.temperature_unit(result.properties.meta.units.air_temperature),
                humiditiy: result.properties.meta.units.relative_humidity,
            },
            current: WeatherCurrent {
                temperature: weather.data.instant.details.air_temperature as f32,
                humiditiy: weather.data.instant.details.relative_humidity as i32,
            },
        })
    }
}

impl LocationApi for Yr {
    fn get_lat_long(&self, name: &str, country: &str) -> Result<Location, WxError> {
        let lr: Vec<OsmLocation> = ureq::get(format!(
            "https://nominatim.openstreetmap.org/search?q={name},{country}&addressdetails=1&format=json&limit=1"
        ))
        .call()?
        .body_mut()
        .read_json()
        .map_err(WxError::Response)?;

        let c = lr.first().ok_or_else(|| WxError::EmptyResult)?.clone();

        Ok(Location {
            name: c.name,
            country_code: c.address.country_code.to_uppercase(),
            latitude: c.lat.parse().unwrap(),
            longitude: c.lon.parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod test {}
