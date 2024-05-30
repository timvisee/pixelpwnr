use image::codecs::gif::GifDecoder;
use rayon::prelude::*;
use std::fs::File;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use crate::pix::canvas::Canvas;
use image::imageops::FilterType;
use image::{AnimationDecoder, DynamicImage};

/// A manager that manages all images to print.
pub struct ImageManager {
    /// Image frames and their preferred delay.
    images: Vec<(DynamicImage, Option<Duration>)>,
    /// Define whether the first image has been drawn
    first: bool,
    index: isize,
}

impl ImageManager {
    /// Intantiate the image manager.
    pub fn from(images: Vec<(DynamicImage, Option<Duration>)>) -> ImageManager {
        ImageManager {
            images,
            first: false,
            index: 0,
        }
    }

    /// Instantiate the image manager, and load the images from the given paths.
    pub fn load(paths: &[&str], size: (u16, u16)) -> ImageManager {
        // Show a status message
        //println!("Load and process {} image(s)...", paths.len());

        // Load the images from the paths
        let image_manager = ImageManager::from(
            paths
                .par_iter()
                .flat_map(|path| load_image(path, size))
                .collect(),
        );

        // TODO: process the image slices

        // We succeeded
        //println!("All images have been loaded successfully");

        image_manager
    }

    /// Tick the image
    ///
    /// Returns the desired duration for othis frame.
    pub fn tick(&mut self, canvas: &mut Canvas) -> Option<Duration> {
        // Get the image index bound
        let bound = self.images.len();

        // Just return if the bound is one, as nothing should be updated
        if self.first && bound == 1 {
            return None;
        }

        // Get the image to use
        let (image, duration) = &mut self.images[self.index as usize % bound];

        // Update the image on the canvas
        canvas.update_image(image);

        // Increase the index
        self.index += 1;

        // We have rendered the first image
        self.first = true;

        *duration
    }

    /// Start working in the image manager.
    ///
    /// This will start walking through all image frames,
    /// and pushes each frame to all painters,
    /// with the specified frames per second.
    pub fn work(&mut self, canvas: &mut Canvas, fps: u32) {
        loop {
            // Determine duration to wait, use frame direction and fall back to FPS
            let frame_delay = self.tick(canvas);
            let delay = frame_delay
                .unwrap_or_else(|| Duration::from_millis((1000f32 / (fps as f32)) as u64));

            // Sleep until we need to show the next image
            sleep(delay);
        }
    }
}

/// Load the image at the given path, and size it correctly
fn load_image(path: &str, size: (u16, u16)) -> Vec<(DynamicImage, Option<Duration>)> {
    // Create a path instance
    let path = Path::new(&path);

    // Check whether the path exists
    if !path.is_file() {
        panic!("The given path does not exist or is not a file");
    }

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    // Load image(s)
    let images = match extension.as_deref() {
        // Load all GIF frames
        Some("gif") => GifDecoder::new(File::open(path).unwrap())
            .expect("failed to decode GIF file")
            .into_frames()
            .collect_frames()
            .expect("failed to parse GIF frames")
            .into_iter()
            .map(|frame| {
                let frame_delay = Duration::from(frame.delay());
                (
                    DynamicImage::ImageRgba8(frame.into_buffer()),
                    Some(frame_delay),
                )
            })
            .collect(),

        // Load single image
        _ => vec![(image::open(path).unwrap(), None)],
    };

    // Resize images to fit the screen
    images
        .into_iter()
        .map(|(image, frame_delay)| {
            (
                image.resize_exact(size.0 as u32, size.1 as u32, FilterType::Gaussian),
                frame_delay,
            )
        })
        .collect()
}
