extern crate bufstream;
extern crate clap;
extern crate image;

use std::io::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use bufstream::BufStream;
use clap::{Arg, App};
use image::{DynamicImage, FilterType, Pixel};



// The default thread count
const DEFAULT_THREAD_COUNT: usize = 4;

// The default image width and height
const DEFAULT_WIDTH: u32 = 1920;
const DEFAULT_HEIGHT: u32 = 1080;

// The default size of the command output read buffer
// const CMD_READ_BUFFER_SIZE: usize = 16;



fn main() {
    // Handle arguments
	let matches = App::new("pixelpwnr")
		.version("0.1")
		.author("Tim Visee <timvisee@gmail.com>")
		.about("Pwns pixelflut")
		.arg(Arg::with_name("HOST")
			.help("The host to pwn \"host:port\"")
            .required(true)
            .index(1))
		.arg(Arg::with_name("count")
			.short("c")
			.long("count")
			.value_name("COUNT")
			.help("Number of simultanious  threads (def: 4)")
			.takes_value(true))
		.arg(Arg::with_name("image")
			.short("i")
			.long("image")
			.value_name("PATH")
			.help("Path of the image to print")
            .required(true)
			.takes_value(true))
		.arg(Arg::with_name("width")
			.short("w")
			.long("width")
			.value_name("PIXELS")
			.help("Drawing width in pixels (def: 1920)")
			.takes_value(true))
		.arg(Arg::with_name("height")
			.short("h")
			.long("height")
			.value_name("PIXELS")
			.help("Drawing height in pixels (def: 1080)")
			.takes_value(true))
		.arg(Arg::with_name("x")
			.short("x")
			.long("x")
			.value_name("PIXELS")
			.help("Drawing X offset in pixels (def: 0)")
			.takes_value(true))
		.arg(Arg::with_name("y")
			.short("y")
			.long("y")
			.value_name("PIXELS")
			.help("Drawing Y offset in pixels (def: 0)")
			.takes_value(true))
		.get_matches();

    // Get the host
    let host = matches
        .value_of("HOST")
        .expect("Please specify a host");

    // Get the count
    let count = matches
        .value_of("count")
        .unwrap_or(&format!("{}", DEFAULT_THREAD_COUNT))
        .parse::<usize>()
        .expect("Invalid count specified");

    // Get the image path
    let image_path = matches
        .value_of("image")
        .expect("Please specify an image path");

    // Get the width and height
    let width = matches
        .value_of("width")
        .unwrap_or(&format!("{}", DEFAULT_WIDTH))
        .parse::<u32>()
        .expect("Invalid image width");
    let height = matches
        .value_of("height")
        .unwrap_or(&format!("{}", DEFAULT_HEIGHT))
        .parse::<u32>()
        .expect("Invalid image height");

    // Get the offset
    let offset_x = matches
        .value_of("x")
        .unwrap_or("0")
        .parse::<u32>()
        .expect("Invalid X offset");
    let offset_y = matches
        .value_of("y")
        .unwrap_or("0")
        .parse::<u32>()
        .expect("Invalid Y offset");

    // Start
    start(
        host,
        image_path, 
        count,
        (width, height),
        (offset_x, offset_y)
    );
}

/// Start the client.
fn start(
    host: &str,
    image_path: &str,
    count: usize,
    size: (u32, u32),
    offset: (u32, u32)
) {
    // Start
    println!("Starting...");

    // Create a new pixelflut canvas
    PixCanvas::new(host, image_path, count, size, offset);

	// Sleep this thread
	thread::sleep(Duration::new(99999999, 0));
}

/// Create a stream to talk to the pixelflut server.
///
/// The stream is returned as result.
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
    host: String,
	painter_count: usize,
    painters: Vec<JoinHandle<u32>>,
    image: DynamicImage,
    size: (u32, u32),
    offset: (u32, u32),
}

impl PixCanvas {
    /// Create a new pixelflut canvas.
    pub fn new(
        host: &str,
        image_path: &str,
        painter_count: usize,
        size: (u32, u32),
        offset: (u32, u32),
    ) -> PixCanvas {
		// Load the image
		let image = load_image(image_path, &size);

        // Initialize the object
        let mut canvas = PixCanvas {
            host: host.to_string(),
			painter_count,
            painters: Vec::with_capacity(painter_count),
            image,
            size,
            offset,
        };

		// Show a status message
		println!("Starting painter threads...");

        // Spawn some painters
        canvas.spawn_painters();

        // Return the canvas
        canvas
    }

    /// Spawn the painters for this canvas
    fn spawn_painters(&mut self) {
        // Spawn some painters
        for i in 0..self.painter_count {
			// Determine the slice width
			let width = self.size.0 / (self.painter_count as u32);

			// Define the area to paint per thread
			let painter_area = Rect::from(
				(i as u32) * width,
				0,
				width,
				self.size.1,
			);

			// Spawn the painter
            self.spawn_painter(painter_area);
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

        // Redefine the offset to make it usable in the thread
        let offset = (self.offset.0, self.offset.1);

        // Create the painter thread
        let thread = thread::spawn(move || {
            // Create a new stream
            let stream = create_stream(host)
                .expect("failed to open stream to pixelflut");

            // Create a new client
            let client = PixClient::new(stream);

            // Create a painter
            let mut painter = Painter::new(client, area, offset, image);

            // Do some work
            loop {
                painter.work().expect("Painter failed to perform work");
            }
        });

        // Add the painter thread to the list
        self.painters.push(thread);
    }
}



struct Painter {
    client: PixClient,
    area: Rect,
    offset: (u32, u32),
    image: DynamicImage,
}

impl Painter {
    /// Create a new painter.
    pub fn new(client: PixClient, area: Rect, offset: (u32, u32), image: DynamicImage) -> Painter {
        Painter {
            client,
            area,
            offset,
            image,
        }
    }

    /// Perform work.
    /// Paint the whole defined area.
    pub fn work(&mut self) -> Result<(), Error> {
        // Get an RGB image
        let image = self.image.to_rgb();

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
                    x + self.area.x + self.offset.0,
                    y + self.area.y + self.offset.1,
                    &color,
                )?;
            }
        }

        // Everything seems to be ok
        Ok(())
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
    fn write_pixel(&mut self, x: u32, y: u32, color: &Color) -> Result<(), Error> {
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
