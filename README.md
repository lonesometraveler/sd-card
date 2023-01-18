# SD Card Manager

This library provides an SD card manager for embedded systems using the `embedded_sdmmc` library. It allows for opening and closing files, and provides an error type for handling errors specific to SD card operations.

## Usage

To use the library, import the `SdCard` struct. Then create a new instance of `SdCard` by passing in an SPI instance and a CS (chip select) pin.

```rust
use sd_card::SdCard;

let mut sd_card = SdCard::new(spi, cs)?;
```

The `SdCard` struct has four main methods:

* `open_file(file_name: &str, mode: Mode) -> Result<File, SdCardError>`: opens a file with the given name and mode (read or write). Returns a `File` struct on success or a `SdCardError` on failure.
* `close_file(f: File) -> Result<(), SdCardError>`: closes a file. Returns a Result indicating success or failure.
* `write(&mut self, f: &mut File, buffer: &[u8]) -> Result<usize, SdCardError>`: Writes the contents of a buffer to the file. Takes a mutable reference to the `File` struct and a slice of bytes as input, and returns the number of bytes written as a `usize` wrapped in a `Result` enum.
* `read(&mut self, f: &mut File, buffer: &mut [u8]) -> Result<usize, SdCardError>`: Reads data from the file and writes it to a buffer. Takes a mutable reference to the `File` struct and a mutable slice of bytes as input, and returns the number of bytes read as a `usize` wrapped in a `Result` enum.


### Errors

Errors are returned as `SdCardError` enum, which can have two variants:

* `SdMmc(embedded_sdmmc::SdMmcError)`: an error from the underlying embedded_sdmmc library.
* `Controller(embedded_sdmmc::Error<embedded_sdmmc::SdMmcError>)`: an error from the `Controller` struct.


## Example

```rust
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
```
