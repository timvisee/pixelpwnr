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
    start(
        arg_handler.host(),
        arg_handler.image_paths(),
        arg_handler.fps(),
        arg_handler.count(),
        arg_handler.size(),
        arg_handler.offset(),
    );
}

/// Start the client.
fn start(
    host: &str,
    image_paths: Vec<&str>,
    fps: u32,
    count: usize,
    size: (u32, u32),
    offset: (u32, u32)
) {
    // Start
    println!("Starting...");

    // Create a new pixelflut canvas
    let mut canvas = PixCanvas::new(host, count, size, offset);

    // Load the image manager
    let mut image_manager = ImageManager::load(image_paths, &size);

    // Animate images
    loop {
        // Tick to use the next image
        image_manager.tick(&mut canvas);

        // Sleep until we need to show the next image
        thread::sleep(
            Duration::from_millis(
                (1000f32 / (fps as f32)) as u64
            )
        );
    }
}
