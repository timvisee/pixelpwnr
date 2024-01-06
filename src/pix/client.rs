use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::net::TcpStream;

use bufstream::BufStream;
use regex::Regex;

use crate::color::Color;

// The default buffer size for reading the client stream.
// - Big enough so we don't have to expand
// - Small enough to not take up to much memory
const CMD_READ_BUFFER_SIZE: usize = 32;

// The response format of the screen size from a pixelflut server.
const PIX_SERVER_SIZE_REGEX: &str = r"^(?i)\s*SIZE\s+([[:digit:]]+)\s+([[:digit:]]+)\s*$";

#[derive(Clone)]
pub enum FlushMode {
    Manual,
    Bytes,
    Commands,
}

/// A pixelflut client.
///
/// This client uses a stream to talk to a pixelflut panel.
/// It allows to write pixels to the panel, and read some status.
///
/// The client provides an interface for other logic to easily talk
/// to the pixelflut panel.
pub struct Client {
    stream: BufStream<TcpStream>,

    /// Whether to use binary mode (PB) instead of (PX).
    binary: bool,

    /// When the stream should be flushed.
    flush_mode: FlushMode,
    current_size: u16,
    flush_size: u16,
}

impl Client {
    /// Create a new client instance.
    pub fn new(stream: TcpStream, binary: bool, flush_mode: FlushMode, flush_size: u16) -> Client {
        Client {
            stream: BufStream::new(stream),
            binary,
            flush_mode,
            flush_size,
            current_size: 0,
        }
    }

    /// Create a new client instane from the given host, and connect to it.
    pub fn connect(
        host: String,
        binary: bool,
        flush_mode: FlushMode,
        flush_size: u16,
    ) -> Result<Client, Error> {
        // Create a new stream, and instantiate the client
        Ok(Client::new(
            create_stream(host)?,
            binary,
            flush_mode,
            flush_size,
        ))
    }

    /// Write a pixel to the given stream.
    pub fn write_pixel(&mut self, x: u16, y: u16, color: Color) -> Result<(), Error> {
        if self.binary {
            let mut data = [
                b'P', b'B',
                // these values will be filled in using to_le_bytes in the next step
                // to account for the machines endianness
                0, // x LSB
                0, // x MSB
                0, // y LSB
                0, // y MSB
                color.r, color.g, color.b, color.a,
            ];
            data[2..4].copy_from_slice(&x.to_le_bytes());
            data[4..6].copy_from_slice(&y.to_le_bytes());
            self.write_command(&data, false)
        } else {
            self.write_command(
                format!("PX {} {} {}", x, y, color.as_hex()).as_bytes(),
                true,
            )
        }
    }

    /// Read the size of the screen.
    pub fn read_screen_size(&mut self) -> Result<(u16, u16), Error> {
        // Read the screen size
        let data = self
            .write_read_command(b"SIZE")
            .expect("Failed to read screen size");

        // Build a regex to parse the screen size
        let re = Regex::new(PIX_SERVER_SIZE_REGEX).unwrap();

        // Find captures in the data, return the result
        match re.captures(&data) {
            Some(matches) => Ok((
                matches[1]
                    .parse::<u16>()
                    .expect("Failed to parse screen width, received malformed data"),
                matches[2]
                    .parse::<u16>()
                    .expect("Failed to parse screen height, received malformed data"),
            )),
            None => Err(Error::new(
                ErrorKind::Other,
                "Failed to parse screen size, received malformed data",
            )),
        }
    }

    /// Flush the write buffer.
    pub fn flush(&mut self) -> Result<(), Error> {
        self.stream.flush()?;
        self.current_size = 0;
        Ok(())
    }

    /// Write the given command to the given stream.
    fn write_command(&mut self, cmd: &[u8], newline: bool) -> Result<(), Error> {
        // Write the pixels and a new line

        let new_size: u16 = match self.flush_mode {
            FlushMode::Manual => {
                self.stream.write_all(cmd)?;
                if newline {
                    self.stream.write_all(b"\n")?;
                }
                return Ok(());
            }
            FlushMode::Bytes => {
                let mut new_size = cmd.len();
                if newline {
                    new_size += 1;
                }
                new_size as u16
            }
            FlushMode::Commands => 1,
        };

        if self.current_size + new_size > self.flush_size {
            self.flush()?;
        }

        self.stream.write_all(cmd)?;
        if newline {
            self.stream.write_all(b"\n")?;
        }
        self.current_size += new_size;

        if self.current_size == self.flush_size {
            self.flush()?;
        }

        Ok(())
    }

    /// Write the given command to the given stream, and read the output.
    fn write_read_command(&mut self, cmd: &[u8]) -> Result<String, Error> {
        // Write the command
        self.write_command(cmd, true)?;

        // Flush the pipe, ensure the command is actually sent
        self.stream.flush()?;

        // Read the output
        // TODO: this operation may get stuck (?) if nothing is received from the server
        let mut buffer = String::with_capacity(CMD_READ_BUFFER_SIZE);
        self.stream.read_line(&mut buffer)?;

        // Return the read string
        Ok(buffer)
    }
}

impl Drop for Client {
    /// Nicely drop the connection when the client is disconnected.
    fn drop(&mut self) {
        let _ = self.write_command(b"\nQUIT", true);
    }
}

/// Create a stream to talk to the pixelflut server.
///
/// The stream is returned as result.
fn create_stream(host: String) -> Result<TcpStream, Error> {
    TcpStream::connect(host)
}
