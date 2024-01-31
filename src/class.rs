use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum Class {
    /// Internet
    #[default]
    #[serde(rename = "IN")]
    IN,
    /// Chaos
    #[serde(rename = "CH")]
    CH,
    /// Hesiod
    #[serde(rename = "HS")]
    HS,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::IN => f.write_str("IN"),
            Class::CH => f.write_str("CH"),
            Class::HS => f.write_str("HS"),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::Class;

    #[test]
    fn deser() {
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct ExampleStruct {
            class: Class,
        }

        let example = vec![ExampleStruct { class: Class::IN }];

        let serialized = serde_yaml::to_string(&example).unwrap();
        println!("{serialized}");

        assert_eq!(
            serde_yaml::from_str::<Vec<ExampleStruct>>(&serialized).unwrap(),
            example
        );
    }
}
