use super::Header;
use super::console::Console;
use super::mirroring::Mirroring;
use super::timing::Timing;
use super::vs_system::{hardware::VsSystemHardware, ppu::VsSystemPPU};
use anyhow::{Result, bail};
use std::io::Read;

/// Encode NES 2.0 file header into `Header` struct
pub struct Encoder {
    buffer: [u8; 16],
}

impl Encoder {
    // Create a new encoder
    pub fn new() -> Self {
        Encoder { buffer: [0; 16] }
    }

    // Encode Header from reader.
    pub fn encode<R: Read>(&mut self, mut r: R) -> Result<Header> {
        r.read_exact(self.buffer.as_mut_slice())?;

        // Checks if it's valid NES 2.0 file format
        if !self.buffer.starts_with(&[0x4e, 0x45, 0x53, 0x1a]) {
            bail!("starts with invalid byte sequence");
        } else if self.buffer[7] & 0b1100 != 0b1000 {
            bail!("invalid NES 2.0 identifier");
        }

        // Encode
        let mapper = self.encode_mapper();
        let submapper = Some(self.encode_submapper());
        let mirroring = Some(self.encode_mirroring());
        let battery = Some(self.encode_battery());
        let trainer = Some(self.encode_trainer());
        let console = Some(self.encode_console()?);
        let timing = Some(self.encode_timing());
        let vs_system_ppu = if console == Some(Console::VsSystem) {
            Some(self.encode_vs_system_ppu()?)
        } else {
            None
        };
        let vs_system_hardware = if console == Some(Console::VsSystem) {
            Some(self.encode_vs_system_hardware()?)
        } else {
            None
        };
        let prg_rom_size = self.encode_prg_rom_size()?;
        let prg_ram_size = Some(self.encode_prg_ram_size()?);
        let prg_nvram_size = Some(self.encode_prg_nvram_size()?);
        let chr_rom_size = self.encode_chr_rom_size()?;
        let chr_ram_size = Some(self.encode_chr_ram_size()?);
        let chr_nvram_size = Some(self.encode_chr_nvram_size()?);

        Ok(Header {
            mapper,
            submapper,
            mirroring,
            battery,
            trainer,
            console,
            timing,
            vs_system_ppu,
            vs_system_hardware,
            prg_rom_size,
            prg_ram_size,
            prg_nvram_size,
            chr_rom_size,
            chr_ram_size,
            chr_nvram_size,
        })
    }

    fn encode_mapper(&self) -> u64 {
        let mut mapper = 0;
        mapper |= ((self.buffer[6] & 0xf0) as u64) >> 4;
        mapper |= (self.buffer[7] & 0xf0) as u64;
        mapper |= ((self.buffer[8] & 0xf) as u64) << 8;
        mapper
    }

    fn encode_submapper(&self) -> u64 {
        ((self.buffer[8] & 0xf0) >> 4) as u64
    }

    fn encode_mirroring(&self) -> Mirroring {
        if self.buffer[6] & 0b1000 != 0 {
            Mirroring::FourScreens
        } else if self.buffer[6] & 0b0001 != 0 {
            Mirroring::Horizontal
        } else {
            Mirroring::Vertical
        }
    }

    fn encode_battery(&self) -> bool {
        self.buffer[6] & 0b10 != 0
    }

    fn encode_trainer(&self) -> bool {
        self.buffer[6] & 0b100 != 0
    }

    fn encode_console(&self) -> Result<Console> {
        match self.buffer[7] & 0b11 {
            0x00 => Ok(Console::Nes),
            0x01 => Ok(Console::VsSystem),
            0x02 => Ok(Console::Playchoice10),
            0x03 => match self.buffer[13] & 0xf {
                0x00 => Ok(Console::Nes),
                0x01 => Ok(Console::VsSystem),
                0x02 => Ok(Console::Playchoice10),
                0x03 => Ok(Console::NesWithDecimal),
                0x04 => Ok(Console::NesWithEPSM),
                0x05 => Ok(Console::VT01),
                0x06 => Ok(Console::VT02),
                0x07 => Ok(Console::VT03),
                0x08 => Ok(Console::VT09),
                0x09 => Ok(Console::VT32),
                0x0a => Ok(Console::VT369),
                0x0b => Ok(Console::UM6539),
                0x0c => Ok(Console::FamicomNetworkSystem),
                0x0d => Ok(Console::Reserved0D),
                0x0e => Ok(Console::Reserved0E),
                0x0f => Ok(Console::Reserved0F),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn encode_timing(&self) -> Timing {
        match self.buffer[12] & 0b11 {
            0 => Timing::RP2C02,
            1 => Timing::RP2C07,
            2 => Timing::Multiple,
            3 => Timing::UA6538,
            _ => unreachable!(),
        }
    }

    fn encode_vs_system_ppu(&self) -> Result<VsSystemPPU> {
        match self.buffer[13] & 0xf {
            0x00 => Ok(VsSystemPPU::Any),
            0x01 => Ok(VsSystemPPU::Reserved01),
            0x02 => Ok(VsSystemPPU::RP2C04_0001),
            0x03 => Ok(VsSystemPPU::RP2C04_0002),
            0x04 => Ok(VsSystemPPU::RP2C04_0003),
            0x05 => Ok(VsSystemPPU::RP2C04_0004),
            0x06 => Ok(VsSystemPPU::Reserved06),
            0x07 => Ok(VsSystemPPU::Reserved07),
            0x08 => Ok(VsSystemPPU::RC2C05_01),
            0x09 => Ok(VsSystemPPU::RC2C05_02),
            0x0a => Ok(VsSystemPPU::RC2C05_03),
            0x0b => Ok(VsSystemPPU::RC2C05_04),
            0x0c => Ok(VsSystemPPU::Reserved0C),
            0x0d => Ok(VsSystemPPU::Reserved0D),
            0x0e => Ok(VsSystemPPU::Reserved0E),
            0x0f => Ok(VsSystemPPU::Reserved0F),
            value => bail!("invalid vs system ppu type: {value}"),
        }
    }

    fn encode_vs_system_hardware(&self) -> Result<VsSystemHardware> {
        match (self.buffer[13] & 0xf0) >> 4 {
            0 => Ok(VsSystemHardware::UnisystemNormal),
            1 => Ok(VsSystemHardware::UnisystemRBIBaseball),
            2 => Ok(VsSystemHardware::UnisystemTKOBoxing),
            3 => Ok(VsSystemHardware::UnisystemSuperXevious),
            4 => Ok(VsSystemHardware::UnisystemIceClimberJapan),
            5 => Ok(VsSystemHardware::DualSystemNormal),
            6 => Ok(VsSystemHardware::DualSystemRaidOnBungelingBay),
            value => bail!("invalid vs system hardware type: {value}"),
        }
    }

    fn encode_prg_rom_size(&self) -> Result<String> {
        let lsb = self.buffer[4];
        let msb = self.buffer[9] & 0xf;
        encode_rom_size(lsb, msb, 16)
    }

    fn encode_prg_ram_size(&self) -> Result<String> {
        let count = self.buffer[10] & 0xf;
        Ok(encode_ram_size(count))
    }

    fn encode_prg_nvram_size(&self) -> Result<String> {
        let count = (self.buffer[10] & 0xf0) >> 4;
        Ok(encode_ram_size(count))
    }

    fn encode_chr_rom_size(&self) -> Result<String> {
        let lsb = self.buffer[5];
        let msb = (self.buffer[9] & 0xf0) >> 4;
        encode_rom_size(lsb, msb, 8)
    }

    fn encode_chr_ram_size(&self) -> Result<String> {
        let count = self.buffer[11] & 0xf;
        Ok(encode_ram_size(count))
    }

    fn encode_chr_nvram_size(&self) -> Result<String> {
        let count = (self.buffer[11] & 0xf0) >> 4;
        Ok(encode_ram_size(count))
    }
}

fn encode_rom_size(lsb: u8, msb: u8, unit_size: u64) -> Result<String> {
    if msb == 0xF {
        let exp = ((lsb & 0b1111_1100) >> 2) as u64;
        let mul = (msb & 0b0000_0011) as u64;
        if exp > 60 {
            bail!("too large rom size");
        }
        Ok(format!("{}", (1 << exp) * (2 * mul + 1)))
    } else {
        let units = lsb as u64 | (msb as u64) << 8;
        let size = units * unit_size;
        if size.is_multiple_of(1024) {
            Ok(format!("{}M", size / 1024))
        } else {
            Ok(format!("{}K", size))
        }
    }
}

fn encode_ram_size(count: u8) -> String {
    if count == 0 {
        "0K".into()
    } else {
        let size = 64 << (count as u64);
        if size % 1024 == 0 {
            format!("{}K", size / 1024)
        } else {
            format!("{}", size)
        }
    }
}
