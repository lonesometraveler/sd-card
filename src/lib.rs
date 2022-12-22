#![no_std]
use embedded_sdmmc::{
    BlockSpi, Controller, Directory, File, Mode, SdMmcSpi, TimeSource, Timestamp, Volume,
};

const MAX_DIRS: usize = 4;
const MAX_FILES: usize = 4;
type BdController<'a, SPI, CS> = Controller<BlockSpi<'a, SPI, CS>, SdMmcClock, MAX_DIRS, MAX_FILES>;
type SdMmcError = embedded_sdmmc::Error<embedded_sdmmc::SdMmcError>;

/// Clock
pub struct SdMmcClock;

impl TimeSource for SdMmcClock {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

/// SD Card manager
pub struct SdCard<SPI, CS>
where
    SPI: embedded_hal::blocking::spi::Transfer<u8>,
    CS: embedded_hal::digital::v2::OutputPin,
    <SPI as embedded_hal::prelude::_embedded_hal_blocking_spi_Transfer<u8>>::Error:
        core::fmt::Debug,
{
    sdmmc_spi: SdMmcSpi<SPI, CS>,
    volume: Volume,
    directory: Directory,
}

impl<SPI, CS> SdCard<SPI, CS>
where
    SPI: embedded_hal::blocking::spi::Transfer<u8>,
    CS: embedded_hal::digital::v2::OutputPin,
    <SPI as embedded_hal::prelude::_embedded_hal_blocking_spi_Transfer<u8>>::Error:
        core::fmt::Debug,
{
    pub fn new(spi: SPI, cs: CS) -> Result<Self, embedded_sdmmc::sdmmc::Error> {
        let mut sdmmc_spi = SdMmcSpi::new(spi, cs);
        let volume: Volume;
        let directory: Directory;

        {
            let block = sdmmc_spi.acquire()?;
            let mut controller: BdController<SPI, CS> = Controller::new(block, SdMmcClock);
            volume = controller.get_volume(embedded_sdmmc::VolumeIdx(0)).unwrap();
            directory = controller.open_root_dir(&volume).unwrap();
        }

        Ok(SdCard {
            sdmmc_spi,
            volume,
            directory,
        })
    }

    /// Opens a file
    pub fn open_file(&mut self, file_name: &str, mode: Mode) -> Result<File, SdMmcError> {
        let block = self.sdmmc_spi.acquire().unwrap();
        let mut controller: BdController<SPI, CS> = Controller::new(block, SdMmcClock);
        controller.open_file_in_dir(&mut self.volume, &self.directory, file_name, mode)
    }

    /// Closes a file
    pub fn close_file(&mut self, f: File) -> Result<(), SdMmcError> {
        let block = self.sdmmc_spi.acquire().unwrap();
        let mut controller: BdController<SPI, CS> = Controller::new(block, SdMmcClock);
        controller.close_file(&self.volume, f)
    }

    /// Writes to an opened file
    pub fn write(&mut self, f: &mut File, buffer: &[u8]) -> Result<usize, SdMmcError> {
        let block = self.sdmmc_spi.acquire().unwrap();
        let mut controller: BdController<SPI, CS> = Controller::new(block, SdMmcClock);
        controller.write(&mut self.volume, f, buffer)
    }

    /// Reads from a file
    pub fn read(&mut self, f: &mut File, buffer: &mut [u8]) -> Result<usize, SdMmcError> {
        let block = self.sdmmc_spi.acquire().unwrap();
        let mut controller: BdController<SPI, CS> = Controller::new(block, SdMmcClock);
        controller.read(&self.volume, f, buffer)
    }

    /// Opens a file, Writes to the file, and finally Closes the file.
    /// Returns the number of bytes written.
    pub fn write_file(&mut self, file_name: &str, buffer: &[u8]) -> Result<usize, SdMmcError> {
        let block = self.sdmmc_spi.acquire().unwrap();
        let volume = &mut self.volume;
        let directory = &self.directory;

        let mut controller: BdController<SPI, CS> = Controller::new(block, SdMmcClock);

        let mut file = controller.open_file_in_dir(
            volume,
            directory,
            file_name,
            Mode::ReadWriteCreateOrAppend,
        )?;

        let n = controller.write(volume, &mut file, buffer)?;

        controller.close_file(volume, file)?;

        Ok(n)
    }

    /// Opens a file, Reads from the file, and finally Closes the file.
    /// Returns the number of bytes read.
    pub fn read_file(&mut self, file_name: &str, buffer: &mut [u8]) -> Result<usize, SdMmcError> {
        let block = self.sdmmc_spi.acquire().unwrap();
        let volume = &mut self.volume;
        let directory = &self.directory;

        let mut controller: BdController<SPI, CS> = Controller::new(block, SdMmcClock);

        let mut file = controller.open_file_in_dir(volume, directory, file_name, Mode::ReadOnly)?;

        let n = controller.read(volume, &mut file, buffer)?;

        controller.close_file(volume, file)?;

        Ok(n)
    }

    /// Closes the directory
    pub fn close_dir(mut self) {
        let block = self.sdmmc_spi.acquire().unwrap();
        let mut controller: BdController<SPI, CS> = Controller::new(block, SdMmcClock);
        controller.close_dir(&self.volume, self.directory)
    }
}
