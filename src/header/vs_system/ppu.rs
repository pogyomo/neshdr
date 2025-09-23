use anyhow::{Error, bail};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(try_from = "String", into = "String")]
pub enum VsSystemPPU {
    Any,
    Reserved01,
    RP2C04_0001,
    RP2C04_0002,
    RP2C04_0003,
    RP2C04_0004,
    Reserved06,
    Reserved07,
    RC2C05_01,
    RC2C05_02,
    RC2C05_03,
    RC2C05_04,
    Reserved0C,
    Reserved0D,
    Reserved0E,
    Reserved0F,
}

impl TryFrom<String> for VsSystemPPU {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Any" => Ok(VsSystemPPU::Any),
            "Reserved01" => Ok(VsSystemPPU::Reserved01),
            "RP2C04-0001" => Ok(VsSystemPPU::RP2C04_0001),
            "RP2C04-0002" => Ok(VsSystemPPU::RP2C04_0002),
            "RP2C04-0003" => Ok(VsSystemPPU::RP2C04_0003),
            "RP2C04-0004" => Ok(VsSystemPPU::RP2C04_0004),
            "Reserved06" => Ok(VsSystemPPU::Reserved06),
            "Reserved07" => Ok(VsSystemPPU::Reserved07),
            "RC2C05-01" => Ok(VsSystemPPU::RC2C05_01),
            "RC2C05-02" => Ok(VsSystemPPU::RC2C05_02),
            "RC2C05-03" => Ok(VsSystemPPU::RC2C05_03),
            "RC2C05-04" => Ok(VsSystemPPU::RC2C05_04),
            "Reserved0C" => Ok(VsSystemPPU::Reserved0C),
            "Reserved0D" => Ok(VsSystemPPU::Reserved0D),
            "Reserved0E" => Ok(VsSystemPPU::Reserved0E),
            "Reserved0F" => Ok(VsSystemPPU::Reserved0F),
            _ => bail!("invalid vs system ppu name: {value}"),
        }
    }
}

impl From<VsSystemPPU> for String {
    fn from(value: VsSystemPPU) -> Self {
        let value = match value {
            VsSystemPPU::Any => "Any",
            VsSystemPPU::Reserved01 => "Reserved01",
            VsSystemPPU::RP2C04_0001 => "RP2C04-0001",
            VsSystemPPU::RP2C04_0002 => "RP2C04-0002",
            VsSystemPPU::RP2C04_0003 => "RP2C04-0003",
            VsSystemPPU::RP2C04_0004 => "RP2C04-0004",
            VsSystemPPU::Reserved06 => "Reserved06",
            VsSystemPPU::Reserved07 => "Reserved07",
            VsSystemPPU::RC2C05_01 => "RC2C05-01",
            VsSystemPPU::RC2C05_02 => "RC2C05-02",
            VsSystemPPU::RC2C05_03 => "RC2C05-03",
            VsSystemPPU::RC2C05_04 => "RC2C05-04",
            VsSystemPPU::Reserved0C => "Reserved0C",
            VsSystemPPU::Reserved0D => "Reserved0D",
            VsSystemPPU::Reserved0E => "Reserved0E",
            VsSystemPPU::Reserved0F => "Reserved0F",
        };
        value.to_string()
    }
}
