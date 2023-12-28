use rayon::prelude::*;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use image;
use image::imageops::FilterType;
use image::DynamicImage;

use pix::canvas::Canvas;

/// A manager that manages all images to print.
pub struct ImageManager {
    images: Vec<DynamicImage>,
    // Define whether the first image has been drawn
    first: bool,
    index: isize,
}

impl ImageManager {
    /// Intantiate the image manager.
    pub fn from(images: Vec<DynamicImage>) -> ImageManager {
        ImageManager {
            images,
            first: false,
            index: 0,
        }
    }

    /// Instantiate the image manager, and load the images from the given paths.
    pub fn load(paths: &[&str], size: (u16, u16)) -> ImageManager {
        // Show a status message
        println!("Load and process {} image(s)...", paths.len());

        // Load the images from the paths
        let image_manager = ImageManager::from(
            paths
                .par_iter()
                .map(|path| load_image(path, size))
                .collect(),
        );

        // TODO: process the image slices

        // We succeeded
        println!("All images have been loaded successfully");

        image_manager
    }

    /// Tick the image
    pub fn tick(&mut self, canvas: &mut Canvas) {
        // Get the image index bound
        let bound = self.images.len();

        // Just return if the bound is one, as nothing should be updated
        if self.first && bound == 1 {
            return;
        }

        // Get the image to use
        let image = &mut self.images[self.index as usize % bound];

        // Update the image on the canvas
        canvas.update_image(image);

        // Increase the index
        self.index += 1;

        // We have rendered the first image
        self.first = true;
    }

    /// Start working in the image manager.
    ///
    /// This will start walking through all image frames,
    /// and pushes each frame to all painters,
    /// with the specified frames per second.
    pub fn work(&mut self, canvas: &mut Canvas, fps: u32) {
        loop {
            // Tick to use the next image
            self.tick(canvas);

            // Sleep until we need to show the next image
            sleep(Duration::from_millis((1000f32 / (fps as f32)) as u64));
        }
    }
}

/// Load the image at the given path, and size it correctly
fn load_image(path: &str, size: (u16, u16)) -> DynamicImage {
    // Create a path instance
    let path = Path::new(&path);

    // Check whether the path exists
    if !path.is_file() {
        panic!("The given path does not exist or is not a file");
    }

    // Load the image
    let image = image::open(&path).unwrap();

    // Resize the image to fit the screen
    image.resize_exact(size.0 as u32, size.1 as u32, FilterType::Gaussian)
}
