extern crate bufstream;
extern crate image;

use std::env;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use std::thread;
use std::thread::JoinHandle;

use bufstream::BufStream;
use image::{GenericImage, DynamicImage, FilterType, Pixel, Primitive};



// The target host
const HOST: &'static str = "127.0.0.1:8080";
// const HOST: &'static str = "151.217.38.83:1234";

// The default size of the command output read buffer
const CMD_READ_BUFFER_SIZE: usize = 32;



fn main() {
    // Start
    println!("Starting...");

    // Define the size to use
    // TODO: get the size from the screen
    let size = (1920u32, 1080u32);

    // Define the image to use
    // TODO: get the image path from the CLI
    let image_path = "/home/timvisee/Pictures/sample.jpg";

    // Create a new pixelflut canvas
    let canvas = PixCanvas::new(HOST, image_path, size, 10);

    loop {}
}

/// Create a stream to talk to the pixelflut server.
///
/// The stream is returned as result.
// TODO: Specify a host here!
fn create_stream(host: String) -> Result<TcpStream, Error> {
    TcpStream::connect(host)
}

/// Load the image at the given path, and size it correctly
fn load_image(path: &str, size: &(u32, u32)) -> DynamicImage {
    // Create a path instance
    let path = Path::new(&path);

    // TODO: make sure the path exists.

    // Load the image
    println!("Loading image...");
    let image = image::open(&path).unwrap();

    // Start processing the image for the screen
    println!("Processing image...");

    // Resize the image to fit the screen
    image.resize_exact(
        size.0,
        size.1,
        FilterType::Gaussian,
    )
}



/// A pixflut instance 
struct PixCanvas {
    host: &'static str,
    size: (u32, u32),
    painters: Vec<JoinHandle<u32>>,
    image: DynamicImage,
}

impl PixCanvas {
    /// Create a new pixelflut canvas.
    pub fn new(host: &'static str, image_path: &str, size: (u32, u32), painter_count: usize) -> PixCanvas {
        // Initialize the object
        let mut canvas = PixCanvas {
            host,
            size,
            painters: Vec::with_capacity(painter_count),
            image: load_image(image_path, &size),
        };

        // Spawn some painters
        canvas.spawn_painters();

        // Return the canvas
        canvas
    }

    /// Spawn the painters for this canvas
    fn spawn_painters(&mut self) {
        // Spawn some painters
        for i in 0..10 {
            self.spawn_painter(Rect::from(i * 100, 0, 100, 1000));
        }
    }

    /// Spawn a single painter in a thread.
    fn spawn_painter(&mut self, area: Rect) {
        // Get the host that will be used
        let host = self.host.to_string();

        // Get the part of the image to draw by this painter
        let image = self.image.crop(
            area.x,
            area.y,
            area.w,
            area.h
        );

        // Create the painter thread
        let thread = thread::spawn(move || {
            // Create a new stream
            let stream = create_stream(host)
                .expect("failed to open stream to pixelflut");

            // Create a new client
            let client = PixClient::new(stream);

            // Create a painter
            let mut painter = Painter::new(client, area, image);

            // Do some work
            loop {
                painter.work();
            }
        });

        // Add the painter thread to the list
        self.painters.push(thread);
    }
}



struct Painter {
    client: PixClient,
    area: Rect,
    image: DynamicImage,
}

impl Painter {
    /// Create a new painter.
    pub fn new(client: PixClient, area: Rect, image: DynamicImage) -> Painter {
        Painter {
            client,
            area,
            image,
        }
    }

    /// Perform work.
    /// Paint the whole defined area.
    pub fn work(&mut self) {
        // Get an RGB image
        let image = self.image.to_rgb();

        // Define the color to draw with
        let color = Color::from(0, 255, 255);

        // Loop through all the pixels, and set their color
        for x in 0..self.area.w {
            for y in 0..self.area.h {
                // Get the pixel at this location
                let pixel = image.get_pixel(x, y);

				// Get the channels
				let channels = pixel.channels();

				// Define the color
				let color = Color::from(
					channels[0],
					channels[1],
					channels[2],
				);

                // Set the pixel
                self.client.write_pixel(
                    x + self.area.x,
                    y + self.area.y,
                    &color,
                );
            }
        }
    }
}



/// A pixelflut client.
/// This client uses a stream to talk to a pixelflut panel.
/// It allows to write pixels to the panel, and read some status.
struct PixClient {
    stream: BufStream<TcpStream>,
}

impl PixClient {
    /// Create a new client instance.
    pub fn new(stream: TcpStream) -> PixClient {
        PixClient {
            stream: BufStream::new(stream),
        }
    }

    /// Write a pixel to the given stream.
    fn write_pixel(&mut self, x: u32, y: u32, color: &Color) {
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

    //     // TODO: Remove this after debugging
    //     println!("Read size: {}", size);
    // }

    /// Write the given command to the given stream.
    fn write_command(&mut self, cmd: String) {
        self.stream.write(cmd.as_bytes());
        self.stream.write("\n".as_bytes());
    }

    // /// Write the given command to the given stream, and read the output.
    // fn write_read_command(&mut self, cmd: String) -> Result<String, Error> {
    //     // Write the command
    //     self.write_command(cmd);

    //     // Read the output
    //     let mut buffer = String::with_capacity(CMD_READ_BUFFER_SIZE);
    //     println!("Reading line...");
    //     self.stream.read_line(&mut buffer)?;
    //     println!("Done reading");

    //     // Return the read string
    //     Ok(buffer)
    // }
}



/// Color structure.
#[derive(Copy, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Create a new color instance
    pub fn from(r: u8, g: u8, b: u8) -> Color {
        Color {
            r,
            g,
            b,
        }
    }

    /// Get a hexadecimal representation of the color,
    /// such as `FFFFFF` for white and `FF0000` for red.
    pub fn as_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}



/// Rectangle struct.
pub struct Rect {
    // TODO: Make these properties private
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn from(x: u32, y: u32, w: u32, h: u32) -> Rect {
        Rect {
            x,
            y,
            w,
            h,
        }
    }
}
