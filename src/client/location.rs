use crate::errors::WxError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LocationResult {
    results: Vec<Location>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    pub name: String,
    pub country_code: String,
    pub latitude: f32,
    pub longitude: f32,
}

pub fn get_lat_long(name: &str, country: &str) -> Result<Location, WxError> {
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

#[cfg(test)]
mod test {}
