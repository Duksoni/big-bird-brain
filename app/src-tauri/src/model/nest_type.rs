use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum NestType {
    Platform,
    Bowl,
    Cavity,
    Ground,
    Star,
}
