use anyhow::{Error, bail};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(try_from = "String", into = "String")]
pub enum VsSystemHardware {
    UnisystemNormal,
    UnisystemRBIBaseball,
    UnisystemTKOBoxing,
    UnisystemSuperXevious,
    UnisystemIceClimberJapan,
    DualSystemNormal,
    DualSystemRaidOnBungelingBay,
}

impl TryFrom<String> for VsSystemHardware {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Default" => Ok(VsSystemHardware::UnisystemNormal),
            "RBI Baseball" => Ok(VsSystemHardware::UnisystemRBIBaseball),
            "TKO Boxing" => Ok(VsSystemHardware::UnisystemTKOBoxing),
            "Super Xevious" => Ok(VsSystemHardware::UnisystemSuperXevious),
            "Ice Climber Japan" => Ok(VsSystemHardware::UnisystemIceClimberJapan),
            "Dual System" => Ok(VsSystemHardware::DualSystemNormal),
            "Raid on Bungeling Bay" => Ok(VsSystemHardware::DualSystemRaidOnBungelingBay),
            _ => bail!("invalid vs system hardware name: {value}"),
        }
    }
}

impl From<VsSystemHardware> for String {
    fn from(value: VsSystemHardware) -> Self {
        let value = match value {
            VsSystemHardware::UnisystemNormal => "Default",
            VsSystemHardware::UnisystemRBIBaseball => "RBI Baseball",
            VsSystemHardware::UnisystemTKOBoxing => "TKO Boxing",
            VsSystemHardware::UnisystemSuperXevious => "Super Xevious",
            VsSystemHardware::UnisystemIceClimberJapan => "Ice Climber Japan",
            VsSystemHardware::DualSystemNormal => "Dual System",
            VsSystemHardware::DualSystemRaidOnBungelingBay => "Raid on Bungeling Bay",
        };
        value.to_string()
    }
}
