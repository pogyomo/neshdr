use anyhow::{Error, bail};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(try_from = "String", into = "String")]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreens,
}

impl TryFrom<String> for Mirroring {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Horizontal" => Ok(Mirroring::Horizontal),
            "Vertical" => Ok(Mirroring::Vertical),
            "FourScreens" => Ok(Mirroring::FourScreens),
            _ => bail!("invalid mirroring name: {value}"),
        }
    }
}

impl From<Mirroring> for String {
    fn from(value: Mirroring) -> Self {
        let value = match value {
            Mirroring::Horizontal => "Horizontal",
            Mirroring::Vertical => "Vertical",
            Mirroring::FourScreens => "FourScreens",
        };
        value.to_string()
    }
}
