extern crate rand;

use rand::distributions::{IndependentSample, Range};

use std::io::Error;
use std::io::prelude::*;
use std::net::TcpStream;

// The target host
const HOST: &'static str = "127.0.0.1:8080";

fn main() {
    // Start
    println!("Starting...");

    let range = Range::new(1, 1000);
    let mut rng = rand::thread_rng();

    // Create the control stream
    let mut stream = create_stream()
        .expect("failed to open control stream");

    // Create a client
    let mut client = PixClient::new(stream);

    // Write a pixel
    loop {
        client.write_pixel(
            range.ind_sample(&mut rng),
            range.ind_sample(&mut rng),
            &Color::from(255, 0, 0)
        );
    }
}

/// Create a stream to talk to the pixelflut server.
///
/// The stream is returned as result.
fn create_stream() -> Result<TcpStream, Error> {
    TcpStream::connect(HOST)
}




/// A pixelflut client.
struct PixClient {
    stream: TcpStream,
}

impl PixClient {
    /// Create a new client instance.
    pub fn new(stream: TcpStream) -> PixClient {
        PixClient {
            stream,
        }
    }

    /// Write a pixel to the given stream.
    fn write_pixel(&mut self, x: u16, y: u16, color: &Color) {
        // Write the command to set a pixel
        self.write_command(
            format!("PX {} {} {}", x, y, color.as_hex()),
        )
    }

    /// Write the given command to the given stream.
    fn write_command(&mut self, cmd: String) {
        self.stream.write(cmd.as_bytes());
        self.stream.write("\n".as_bytes());
    }
}



/// Color structure.
#[derive(Copy, Clone)]
struct Color {
    r: u16,
    g: u16,
    b: u16,
}

impl Color {
    /// Create a new color instance
    pub fn from(r: u16, g: u16, b: u16) -> Color {
        Color {
            r,
            g,
            b,
        }
    }

    /// Convert the color to a hexadecimal representation.
    pub fn as_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
