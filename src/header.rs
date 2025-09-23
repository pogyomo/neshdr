pub mod console;
pub mod decoder;
pub mod encoder;
pub mod mirroring;
pub mod timing;
pub mod vs_system;

use crate::header::decoder::Decoder;

use self::console::Console;
use self::encoder::Encoder;
use self::mirroring::Mirroring;
use self::timing::Timing;
use self::vs_system::{hardware::VsSystemHardware, ppu::VsSystemPPU};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub mapper: u64,
    pub submapper: Option<u64>,
    pub mirroring: Option<Mirroring>,
    pub battery: Option<bool>,
    pub trainer: Option<bool>,
    pub console: Option<Console>,
    pub timing: Option<Timing>,
    pub vs_system_ppu: Option<VsSystemPPU>,
    pub vs_system_hardware: Option<VsSystemHardware>,
    pub prg_rom_size: String,
    pub prg_ram_size: Option<String>,
    pub prg_nvram_size: Option<String>,
    pub chr_rom_size: String,
    pub chr_ram_size: Option<String>,
    pub chr_nvram_size: Option<String>,
}

impl Header {
    pub fn from_json<R: Read>(r: R) -> Result<Header> {
        Ok(serde_json::from_reader(r)?)
    }

    pub fn into_json<W: Write>(self, w: W) -> Result<()> {
        Ok(serde_json::to_writer_pretty(w, &self)?)
    }

    pub fn from_bytes<R: Read>(r: R) -> Result<Header> {
        Encoder::new().encode(r)
    }

    pub fn into_bytes<W: Write>(self, w: W) -> Result<()> {
        Decoder::new(self).decode(w)
    }
}
