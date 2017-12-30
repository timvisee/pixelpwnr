use std::path::Path;

use image;
use image::{DynamicImage, FilterType};

use pix_canvas::PixCanvas;



/// A manager that manages all images to print.
pub struct ImageManager {
    images: Vec<DynamicImage>,
    index: isize,
}

impl ImageManager {
    /// Intantiate the image manager.
    pub fn from(images: Vec<DynamicImage>) -> ImageManager {
        ImageManager {
            images,
            index: 0,
        }
    }

    /// Instantiate the image manager, and load the images from the given paths.
    pub fn load(paths: Vec<&str>, size: &(u32, u32)) -> ImageManager {
        // Show a status message
        println!("Load and process {} image(s)...", paths.len());

        // Load the images from the paths
        let image_manager = ImageManager::from(
            paths.iter()
                .map(|path| load_image(path, &size))
                .collect()
        );

        // We succeeded
        println!("All images have been loaded successfully");

        image_manager
    }

    /// Tick the image 
    pub fn tick(&mut self, canvas: &mut PixCanvas) {
        // Get the image index bound
        let bound = self.images.len();

        // Get the image to use
        let image = &mut self.images[
            self.index as usize % bound
        ];

        // Update the image on the canvas
        canvas.update_image(image);

        // Increase the index
        self.index += 1;
    }
}



/// Load the image at the given path, and size it correctly
fn load_image(path: &str, size: &(u32, u32)) -> DynamicImage {
    // Create a path instance
    let path = Path::new(&path);

    // Check whether the path exists
    if !path.is_file() {
        panic!("The given path does not exist or is not a file");
    }

    // Load the image
    let image = image::open(&path).unwrap();

    // Resize the image to fit the screen
    image.resize_exact(
        size.0,
        size.1,
        FilterType::Gaussian,
    )
}
