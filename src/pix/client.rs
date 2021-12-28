extern crate bufstream;
extern crate regex;

use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::net::TcpStream;

use self::bufstream::BufStream;
use self::regex::Regex;

use color::Color;

// The default buffer size for reading the client stream.
// - Big enough so we don't have to expand
// - Small enough to not take up to much memory
const CMD_READ_BUFFER_SIZE: usize = 32;

// The response format of the screen size from a pixelflut server.
const PIX_SERVER_SIZE_REGEX: &str = r"^(?i)\s*SIZE\s+([[:digit:]]+)\s+([[:digit:]]+)\s*$";

/// A pixelflut client.
///
/// This client uses a stream to talk to a pixelflut panel.
/// It allows to write pixels to the panel, and read some status.
///
/// The client provides an interface for other logic to easily talk
/// to the pixelflut panel.
pub struct Client {
    stream: BufStream<TcpStream>,
}

impl Client {
    /// Create a new client instance.
    pub fn new(stream: TcpStream) -> Client {
        Client {
            stream: BufStream::new(stream),
        }
    }

    /// Create a new client instane from the given host, and connect to it.
    pub fn connect(host: String) -> Result<Client, Error> {
        // Create a new stream, and instantiate the client
        Ok(Client::new(create_stream(host)?))
    }

    /// Write a pixel to the given stream.
    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) -> Result<(), Error> {
        // Write the command to set a pixel
        self.write_command(&format!("PX {} {} {}", x, y, color.as_hex()))
    }

    /// Read the size of the screen.
    pub fn read_screen_size(&mut self) -> Result<(u32, u32), Error> {
        // Read the screen size
        let data = self
            .write_read_command("SIZE".into())
            .expect("Failed to read screen size");

        // Build a regex to parse the screen size
        let re = Regex::new(PIX_SERVER_SIZE_REGEX).unwrap();

        // Find captures in the data, return the result
        match re.captures(&data) {
            Some(matches) => Ok((
                matches[1]
                    .parse::<u32>()
                    .expect("Failed to parse screen width, received malformed data"),
                matches[2]
                    .parse::<u32>()
                    .expect("Failed to parse screen height, received malformed data"),
            )),
            None => Err(Error::new(
                ErrorKind::Other,
                "Failed to parse screen size, received malformed data",
            )),
        }
    }

    /// Write the given command to the given stream.
    fn write_command(&mut self, cmd: &str) -> Result<(), Error> {
        // Write the pixels and a new line
        self.stream.write_all(cmd.as_bytes())?;
        self.stream.write_all(b"\n")?;

        // Flush, make sure to clear the send buffer
        // TODO: only flush each 100 pixels?
        // TODO: make flushing configurable?
        // TODO: make buffer size configurable?
        self.stream
            .flush()?;

        // Everything seems to be ok
        Ok(())
    }

    /// Write the given command to the given stream, and read the output.
    fn write_read_command(&mut self, cmd: &str) -> Result<String, Error> {
        // Write the command
        self.write_command(cmd)?;

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
        let _ = self.write_command("\nQUIT".into());
    }
}

/// Create a stream to talk to the pixelflut server.
///
/// The stream is returned as result.
fn create_stream(host: String) -> Result<TcpStream, Error> {
    TcpStream::connect(host)
}
