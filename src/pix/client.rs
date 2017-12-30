extern crate bufstream;

use std::io::Error;
use std::io::prelude::*;
use std::net::TcpStream;

use self::bufstream::BufStream;

use color::Color;



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

    /// Write a pixel to the given stream.
    pub fn write_pixel(&mut self, x: u32, y: u32, color: &Color) -> Result<(), Error> {
        // Write the command to set a pixel
        self.write_command(
            format!("PX {} {} {}", x, y, color.as_hex()),
        )
    }

    // /// Read the size of the screen.
    // fn read_screen_size(&mut self) {
    //     // Read the screen size
    //     let size = self
    //         .write_read_command("SIZE".into())
    //         .expect("Failed to read screen size");
    //
    //     // TODO: Remove this after debugging
    //     println!("Read size: {}", size);
    // }

    /// Write the given command to the given stream.
    fn write_command(&mut self, cmd: String) -> Result<(), Error> {
        // Write the pixels and a new line
        self.stream.write(cmd.as_bytes())?;
        self.stream.write("\n".as_bytes())?;

        // Everything seems to be ok
        Ok(())
    }

    // /// Write the given command to the given stream, and read the output.
    // fn write_read_command(&mut self, cmd: String) -> Result<String, Error> {
    //     // Write the command
    //     self.write_command(cmd);
    //
    //     // Read the output
    //     let mut buffer = String::with_capacity(CMD_READ_BUFFER_SIZE);
    //     println!("Reading line...");
    //     self.stream.read_line(&mut buffer)?;
    //     println!("Done reading");
    //
    //     // Return the read string
    //     Ok(buffer)
    // }
}
