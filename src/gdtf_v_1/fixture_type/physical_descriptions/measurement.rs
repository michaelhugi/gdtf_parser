use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Measurement {
    #[serde(rename = "MeasurementPoint")]
    pub measurement_points: Option<Option<Vec<MeasurementPoint>>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MeasurementPoint {}