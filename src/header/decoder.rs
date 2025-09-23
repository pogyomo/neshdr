use super::Header;
use super::console::Console;
use super::mirroring::Mirroring;
use super::timing::Timing;
use super::vs_system::{hardware::VsSystemHardware, ppu::VsSystemPPU};
use anyhow::{Context, Result, bail};
use std::io::Write;

/// Decode `Header` into NES 2.0 file header
pub struct Decoder {
    buffer: [u8; 16],
    header: Header,
}

impl Decoder {
    // Create a new encoder
    pub fn new(header: Header) -> Self {
        Decoder {
            buffer: [0; 16],
            header,
        }
    }

    // Decode Header into writer
    pub fn decode<W: Write>(&mut self, mut w: W) -> Result<()> {
        // Write NES 2.0 specific values
        self.buffer[0] = 0x4e;
        self.buffer[1] = 0x45;
        self.buffer[2] = 0x53;
        self.buffer[3] = 0x1a;
        self.buffer[7] = 0b1000;

        // Decode
        self.decode_mapper()?;
        self.decode_submapper()?;
        self.decode_mirroring();
        self.decode_battery();
        self.decode_trainer();
        self.decode_console();
        self.decode_timing();
        if self.header.console == Some(Console::VsSystem) {
            self.decode_vs_system_ppu();
            self.decode_vs_system_hardware();
        }
        self.decode_prg_rom_size()?;
        self.decode_prg_ram_size()?;
        self.decode_prg_nvram_size()?;
        self.decode_chr_rom_size()?;
        self.decode_chr_ram_size()?;
        self.decode_chr_nvram_size()?;

        w.write_all(&self.buffer)?;
        Ok(())
    }

    fn decode_mapper(&mut self) -> Result<()> {
        if self.header.mapper > 0xFFF {
            bail!("mapper number is too large: {}", self.header.mapper)
        }
        self.buffer[6] |= ((self.header.mapper & 0xF) << 4) as u8;
        self.buffer[7] |= (self.header.mapper & 0xF0) as u8;
        self.buffer[8] |= ((self.header.mapper & 0xF00) >> 8) as u8;
        Ok(())
    }

    fn decode_submapper(&mut self) -> Result<()> {
        let submapper = self.header.submapper.unwrap_or(0);
        if submapper > 0xF {
            bail!("submapper number is too large: {}", submapper)
        }
        self.buffer[8] |= (submapper << 4) as u8;
        Ok(())
    }

    fn decode_mirroring(&mut self) {
        let mirroring = self.header.mirroring.unwrap_or(Mirroring::Vertical);
        self.buffer[6] |= match mirroring {
            Mirroring::Vertical => 0b0000,
            Mirroring::Horizontal => 0b0001,
            Mirroring::FourScreens => 0b1000,
        };
    }

    fn decode_battery(&mut self) {
        let battery = self.header.battery.unwrap_or(false);
        self.buffer[6] |= if battery { 0b10 } else { 0 };
    }

    fn decode_trainer(&mut self) {
        let trainer = self.header.trainer.unwrap_or(false);
        self.buffer[6] |= if trainer { 0b100 } else { 0 };
    }

    fn decode_console(&mut self) {
        let console = self.header.console.unwrap_or(Console::Nes);
        match console {
            Console::Nes => self.buffer[7] |= 0b00,
            Console::VsSystem => self.buffer[7] |= 0b01,
            Console::Playchoice10 => self.buffer[7] |= 0b10,
            _ => {
                self.buffer[7] |= 0b11;
                self.buffer[13] |= match console {
                    Console::Nes => 0x00,
                    Console::VsSystem => 0x01,
                    Console::Playchoice10 => 0x02,
                    Console::NesWithDecimal => 0x03,
                    Console::NesWithEPSM => 0x04,
                    Console::VT01 => 0x05,
                    Console::VT02 => 0x06,
                    Console::VT03 => 0x07,
                    Console::VT09 => 0x08,
                    Console::VT32 => 0x09,
                    Console::VT369 => 0x0a,
                    Console::UM6539 => 0x0b,
                    Console::FamicomNetworkSystem => 0x0c,
                    Console::Reserved0D => 0x0d,
                    Console::Reserved0E => 0x0e,
                    Console::Reserved0F => 0x0f,
                };
            }
        };
    }

    fn decode_timing(&mut self) {
        let timing = self.header.timing.unwrap_or(Timing::RP2C02);
        self.buffer[12] |= match timing {
            Timing::RP2C02 => 0b00,
            Timing::RP2C07 => 0b01,
            Timing::Multiple => 0b10,
            Timing::UA6538 => 0b11,
        };
    }

    fn decode_vs_system_ppu(&mut self) {
        let ppu = self.header.vs_system_ppu.unwrap_or(VsSystemPPU::Any);
        self.buffer[13] |= match ppu {
            VsSystemPPU::Any => 0x00,
            VsSystemPPU::Reserved01 => 0x01,
            VsSystemPPU::RP2C04_0001 => 0x02,
            VsSystemPPU::RP2C04_0002 => 0x03,
            VsSystemPPU::RP2C04_0003 => 0x04,
            VsSystemPPU::RP2C04_0004 => 0x05,
            VsSystemPPU::Reserved06 => 0x06,
            VsSystemPPU::Reserved07 => 0x07,
            VsSystemPPU::RC2C05_01 => 0x08,
            VsSystemPPU::RC2C05_02 => 0x09,
            VsSystemPPU::RC2C05_03 => 0x0a,
            VsSystemPPU::RC2C05_04 => 0x0b,
            VsSystemPPU::Reserved0C => 0x0c,
            VsSystemPPU::Reserved0D => 0x0d,
            VsSystemPPU::Reserved0E => 0x0e,
            VsSystemPPU::Reserved0F => 0x0f,
        };
    }

    fn decode_vs_system_hardware(&mut self) {
        let hardware = self
            .header
            .vs_system_hardware
            .unwrap_or(VsSystemHardware::UnisystemNormal);
        self.buffer[13] |= match hardware {
            VsSystemHardware::UnisystemNormal => 0x00,
            VsSystemHardware::UnisystemRBIBaseball => 0x10,
            VsSystemHardware::UnisystemTKOBoxing => 0x20,
            VsSystemHardware::UnisystemSuperXevious => 0x30,
            VsSystemHardware::UnisystemIceClimberJapan => 0x40,
            VsSystemHardware::DualSystemNormal => 0x50,
            VsSystemHardware::DualSystemRaidOnBungelingBay => 0x60,
        };
    }

    fn decode_prg_rom_size(&mut self) -> Result<()> {
        let (lsb, msb) = decode_rom_size(&self.header.prg_rom_size, 16)?;
        self.buffer[4] = lsb;
        self.buffer[9] |= msb;
        Ok(())
    }

    fn decode_prg_ram_size(&mut self) -> Result<()> {
        if let Some(ref size) = self.header.prg_ram_size {
            let count = decode_ram_size(size)?;
            self.buffer[10] |= count;
        }
        Ok(())
    }

    fn decode_prg_nvram_size(&mut self) -> Result<()> {
        if let Some(ref size) = self.header.prg_nvram_size {
            let count = decode_ram_size(size)?;
            self.buffer[10] |= count << 4;
        }
        Ok(())
    }

    fn decode_chr_rom_size(&mut self) -> Result<()> {
        let (lsb, msb) = decode_rom_size(&self.header.chr_rom_size, 8)?;
        self.buffer[5] = lsb;
        self.buffer[9] |= msb << 4;
        Ok(())
    }

    fn decode_chr_ram_size(&mut self) -> Result<()> {
        if let Some(ref size) = self.header.chr_ram_size {
            let count = decode_ram_size(size)?;
            self.buffer[11] |= count;
        }
        Ok(())
    }

    fn decode_chr_nvram_size(&mut self) -> Result<()> {
        if let Some(ref size) = self.header.chr_nvram_size {
            let count = decode_ram_size(size)?;
            self.buffer[11] |= count << 4;
        }
        Ok(())
    }
}

/// Returns lsb and msb in this order
fn decode_rom_size(size: &str, unit_size: u64) -> Result<(u8, u8)> {
    let byte = parse_size_as_byte(size)?;
    if byte % 1024 != 0 || (byte / 1024) % unit_size != 0 || ((byte / 1024) / unit_size) > 0xEFF {
        for exp in 0..60 {
            for mul in 0..=3 {
                if byte == (1 << exp) * (mul * 2 + 1) {
                    let lsb = ((exp << 2) as u8) | mul as u8;
                    return Ok((lsb, 0xF));
                }
            }
        }
        bail!("invalid rom size");
    } else {
        let units = ((byte / 1024) / unit_size) as u16;
        let le = units.to_le_bytes();
        Ok((le[0], le[1]))
    }
}

/// Returns shift count
fn decode_ram_size(size: &str) -> Result<u8> {
    let bytes = parse_size_as_byte(size)?;
    for count in 0x00..=0xF {
        if (count == 0 && count as u64 == bytes) || (64 << count) as u64 == bytes {
            return Ok(count);
        }
    }
    bail!("invalid ram size");
}

// Parse size as bytes
fn parse_size_as_byte(size: &str) -> Result<u64> {
    if size.ends_with("K") {
        let n = size.trim_end_matches("K").parse::<u64>()?;
        let n = n
            .checked_mul(1024)
            .with_context(|| "too big size literal".to_string())?;
        Ok(n)
    } else if size.ends_with("M") {
        let n = size.trim_end_matches("M").parse::<u64>()?;
        let n = n
            .checked_mul(1024 * 1024)
            .with_context(|| "too big size literal".to_string())?;
        Ok(n)
    } else {
        let n = size.parse::<u64>()?;
        Ok(n)
    }
}
