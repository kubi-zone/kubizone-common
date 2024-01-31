use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Class {
    #[default]
    #[serde(rename = "IN")]
    Internet,
    #[serde(rename = "CH")]
    Chaos,
    #[serde(rename = "HS")]
    Hesiod
}