extern crate clap;
extern crate image;

mod app;
mod arg_handler;
mod color;
mod image_manager;
mod painter;
mod painter_handle;
mod pix_canvas;
mod pix_client;
mod rect;

use arg_handler::ArgHandler;
use image_manager::ImageManager;
use pix_canvas::PixCanvas;



/// Main application entrypoint.
fn main() {
    // Parse CLI arguments
    let arg_handler = ArgHandler::parse();

    // Start
    start(&arg_handler);
}

/// Start pixelflutting.
fn start<'a>(arg_handler: &ArgHandler<'a>) {
    // Start
    println!("Starting...");

    // Create a new pixelflut canvas
    let mut canvas = PixCanvas::new(
        arg_handler.host(),
        arg_handler.count(),
        arg_handler.size(),
        arg_handler.offset(),
    );

    // Load the image manager
    let mut image_manager = ImageManager::load(
        arg_handler.image_paths(),
        &arg_handler.size(),
    );

    // Start the work in the image manager, to walk through the frames
    image_manager.work(&mut canvas, arg_handler.fps());
}
