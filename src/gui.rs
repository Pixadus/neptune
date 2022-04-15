// Description: file used to set up and manage the program window. 

use eframe::{egui};

pub struct NeptuneApp {
    // Current directory
    directory: String,

    // Vector containing valid files in directory
    images: Vec<String>,

    // Current index of images
    im_index: i32,
}

// If we specify arguments, then try to open them; otherwise, file browser dialog