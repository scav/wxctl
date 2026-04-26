use crate::errors::WxError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LocationResult {
    pub results: Vec<Location>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    pub name: String,
    pub country_code: String,
    pub latitude: f32,
    pub longitude: f32,
}

pub trait LocationApi {
    fn get_lat_long(&self, name: &str, country: &str) -> Result<Location, WxError>;
}

#[cfg(test)]
mod test {}
