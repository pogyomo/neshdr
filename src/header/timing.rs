use anyhow::{Error, bail};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(try_from = "String", into = "String")]
pub enum Timing {
    RP2C02,
    RP2C07,
    Multiple,
    UA6538,
}

impl TryFrom<String> for Timing {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "RP2C02" => Ok(Self::RP2C02),
            "RP2C07" => Ok(Self::RP2C07),
            "Multiple" => Ok(Self::Multiple),
            "UA6538" => Ok(Self::UA6538),
            _ => bail!("invalid timing name: {value}"),
        }
    }
}

impl From<Timing> for String {
    fn from(value: Timing) -> Self {
        let value = match value {
            Timing::RP2C02 => "RP2C02",
            Timing::RP2C07 => "RP2C07",
            Timing::Multiple => "Multiple",
            Timing::UA6538 => "UA6538",
        };
        value.to_string()
    }
}
