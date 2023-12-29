use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use image::DynamicImage;

use crate::rect::Rect;

/// A handle to a painter thread.
///
/// This also holds a channel to the painter thread,
/// to allow image updates to be pushed to the thread.
pub struct Handle {
    #[allow(dead_code)]
    thread: JoinHandle<u32>,
    area: Rect,
    image_sender: Sender<DynamicImage>,
}

impl Handle {
    /// Create a new handle from the given properties.
    pub fn new(thread: JoinHandle<u32>, area: Rect, image_sender: Sender<DynamicImage>) -> Handle {
        Handle {
            thread,
            area,
            image_sender,
        }
    }

    /// Push an image update.
    pub fn update_image(&self, full_image: &mut DynamicImage) {
        // Crop the image to the area
        let image = full_image.crop(
            self.area.x as u32,
            self.area.y as u32,
            self.area.w as u32,
            self.area.h as u32,
        );

        // Push a new image to the thread
        // TODO: return this result
        self.image_sender
            .send(image)
            .expect("Failed to send image update to painter");
    }
}
