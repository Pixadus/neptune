// Name: Neptune
// Author: Parker Lamb
// Date: 4.14.22
// Description: Lightweight and speedy image-viewing application. 

mod functions;
mod input;
mod gui;

fn main() {
    // 0. Base parameters
    let supported_formats = vec!["jpg","JPG","jpeg","JPEG","png","PNG"];

    // 1. Get application CLI arguments
    let arguments = input::get_args();

    // 2. Create application instance
    let dir = functions::get_dir(&arguments);
    
    // 3. Open GTK window, and hand off management to it
    // ** Open image data in window if provided
}