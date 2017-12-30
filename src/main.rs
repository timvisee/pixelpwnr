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

use std::thread;
use std::time::Duration;

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

    // Animate images
    loop {
        // Tick to use the next image
        image_manager.tick(&mut canvas);

        // Sleep until we need to show the next image
        thread::sleep(
            Duration::from_millis(
                (1000f32 / (arg_handler.fps() as f32)) as u64
            )
        );
    }
}
