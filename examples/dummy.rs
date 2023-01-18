use sd_card::SdCard;

fn main() {
    let spi = DummySpi;
    let cs = DummyCsPin;

    let mut sd_card = SdCard::new(spi, cs).unwrap();

    let file_name = "test.txt";
    let mut file = sd_card
        .open_file(file_name, embedded_sdmmc::Mode::ReadWriteCreateOrAppend)
        .unwrap();

    let n = sd_card.write(&mut file, &[1; 256]).unwrap();
    println!("wrote {} bytes", n);

    let mut buffer = [0u8; 64];
    let n = sd_card.read(&mut file, &mut buffer).unwrap();
    println!("read {} bytes\n{:?}", n, &buffer[0..n]);

    sd_card.close_file(file).unwrap();

    sd_card.close_dir().unwrap();
}

struct DummySpi;
struct DummyCsPin;
struct DummyTimeSource;

impl embedded_hal::blocking::spi::Transfer<u8> for DummySpi {
    type Error = ();
    fn transfer<'w>(&mut self, _data: &'w mut [u8]) -> Result<&'w [u8], ()> {
        Ok(&[0])
    }
}

impl embedded_hal::digital::v2::OutputPin for DummyCsPin {
    type Error = ();
    fn set_low(&mut self) -> Result<(), ()> {
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), ()> {
        Ok(())
    }
}

impl embedded_sdmmc::TimeSource for DummyTimeSource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp::from_fat(0, 0)
    }
}
