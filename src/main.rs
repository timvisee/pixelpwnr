use std::io::Error;
use std::io::prelude::*;
use std::net::TcpStream;

// The target host
const HOST: &'static str = "127.0.0.1:8080";

fn main() {
    // Start
    println!("Starting...");

    // Create the control stream
    let mut stream = create_stream()
        .expect("failed to open control stream");

    // Write a pixel
    write_pixel(&mut stream, 100, 100, &Color::from(255, 0, 0));

    // TODO: Loop for now as the stream shouldn't close because the application ends
    loop {}
}

/// Create a stream to talk to the pixelflut server.
///
/// The stream is returned as result.
fn create_stream() -> Result<TcpStream, Error> {
    TcpStream::connect(HOST)
}

/// Write a pixel to the given stream.
fn write_pixel(stream: &mut TcpStream, x: u16, y: u16, color: &Color) {
    // Write the command to set a pixel
    write_command(
        stream,
        format!("PX {} {} {}", x, y, color.as_hex()),
    )
}

/// Write the given command to the given stream.
fn write_command(stream: &mut TcpStream, cmd: String) {
    stream.write(cmd.as_bytes());
    stream.write("\n".as_bytes());
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
        format!("{:X}{:X}{:X}", self.r, self.g, self.b)
    }
}
