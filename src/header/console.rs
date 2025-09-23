use anyhow::{Error, bail};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
pub enum Console {
    Nes,
    VsSystem,
    Playchoice10,
    NesWithDecimal,
    NesWithEPSM,
    VT01,
    VT02,
    VT03,
    VT09,
    VT32,
    VT369,
    UM6539,
    FamicomNetworkSystem,
    Reserved0D,
    Reserved0E,
    Reserved0F,
}

impl TryFrom<String> for Console {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "NES" => Ok(Console::Nes),
            "Vs System" => Ok(Console::VsSystem),
            "Playchoice 10" => Ok(Console::Playchoice10),
            "NES With Decimal" => Ok(Console::NesWithDecimal),
            "NES With EPSM" => Ok(Console::NesWithEPSM),
            "VT01" => Ok(Console::VT01),
            "VT02" => Ok(Console::VT02),
            "VT03" => Ok(Console::VT03),
            "VT09" => Ok(Console::VT09),
            "VT32" => Ok(Console::VT32),
            "VT369" => Ok(Console::VT369),
            "UM6539" => Ok(Console::UM6539),
            "Famicom Network System" => Ok(Console::FamicomNetworkSystem),
            "Reserved0D" => Ok(Console::Reserved0D),
            "Reserved0E" => Ok(Console::Reserved0E),
            "Reserved0F" => Ok(Console::Reserved0F),
            _ => bail!("invalid console name: {value}"),
        }
    }
}

impl From<Console> for String {
    fn from(value: Console) -> Self {
        let value = match value {
            Console::Nes => "NES",
            Console::VsSystem => "Vs System",
            Console::Playchoice10 => "Playchoice 10",
            Console::NesWithDecimal => "NES With Decimal",
            Console::NesWithEPSM => "NES With EPSM",
            Console::VT01 => "VT01",
            Console::VT02 => "VT02",
            Console::VT03 => "VT03",
            Console::VT09 => "VT09",
            Console::VT32 => "VT32",
            Console::VT369 => "VT369",
            Console::UM6539 => "UM6539",
            Console::FamicomNetworkSystem => "Famicom Network System",
            Console::Reserved0D => "Reserved0D",
            Console::Reserved0E => "Reserved0E",
            Console::Reserved0F => "Reserved0F",
        };
        value.to_string()
    }
}
