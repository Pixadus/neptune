// Name: Neptune
// Author: Parker Lamb
// Date: 4.14.22
// Description: Lightweight and speedy image-viewing application. 

mod functions;
mod input;
mod gui;

fn main() {
    // 1. Get application CLI arguments
    let arguments = input::get_args();

    // 2. Determine if we should open a directory, a file, or nothing at all
    let dir = functions::get_dir(&arguments);

    // 3. Scan files in directory
    let files;
    let index;
    match dir {
        Some(d) => {
            files = functions::get_files(&d);
            match files {
                Some(ref f) => index = functions::get_index(&arguments, &f),
                None => index = None
            }
        },
        None => {files = None; index = None;}
    }

    match files {
        Some(f) => {
            match index {
                Some(i) => println!("{:?}",f[i]),
                None => println!("Directory specified")
            }
        }
        None => println!("Nothing specified")
    }
    
    // 3. Open GTK window, and hand off management to it
    // ** Open image data in window if provided
}