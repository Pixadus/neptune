// Name: Neptune
// Author: Parker Lamb
// Date: 7.12.21
// Description: Lightweight and speedy image-viewing application. 

// Main.rs should have as few lines as possible; delegate functions to modules. 

mod input;
mod gui;

fn main() {
    // 0. Base parameters
    let supported_formats = vec!["jpg","JPG","jpeg","JPEG","png","PNG"];

    // 1. Get application CLI arguments
    let arguments = input::get_args();

    // 2. Create GTK application
    let app = gui::setup();
    
    // 3. Open GTK window, and hand off management to it
    // ** Open image data in window if provided
    gui::manage_window(&app, arguments, supported_formats);
}