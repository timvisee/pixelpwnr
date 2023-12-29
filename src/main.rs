extern crate clap;
extern crate image;
extern crate rayon;

mod arg_handler;
mod color;
mod image_manager;
mod painter;
mod pix;
mod rect;

use std::io::Error;

use arg_handler::ArgHandler;
use image_manager::ImageManager;
use pix::canvas::Canvas;
use pix::client::Client;

/// Main application entrypoint.
fn main() {
    // Parse CLI arguments
    let arg_handler = ArgHandler::parse();

    // Start
    start(&arg_handler);
}

/// Start pixelflutting.
fn start(arg_handler: &ArgHandler) {
    // Start
    println!("Starting... (use CTRL+C to stop)");

    // Gather facts about the host
    let screen_size =
        gather_host_facts(arg_handler).expect("Failed to gather facts about pixelflut server");

    // Determine the size to use
    let size = arg_handler.size(Some(screen_size));

    // Create a new pixelflut canvas
    let mut canvas = Canvas::new(
        arg_handler.host(),
        arg_handler.count(),
        size,
        arg_handler.offset(),
        arg_handler.binary(),
        arg_handler.flush(),
    );

    // Load the image manager
    let mut image_manager = ImageManager::load(&arg_handler.image_paths(), size);

    // Start the work in the image manager, to walk through the frames
    image_manager.work(&mut canvas, arg_handler.fps());
}

/// Gather important facts about the host.
fn gather_host_facts(arg_handler: &ArgHandler) -> Result<(u16, u16), Error> {
    // Set up a client, and get the screen size
    let size = Client::connect(arg_handler.host().to_string(), false, false)?.read_screen_size()?;

    // Print status
    println!("Gathered screen size: {}x{}", size.0, size.1);

    Ok(size)
}
