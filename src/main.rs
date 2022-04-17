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
        Some(ref d) => {
            files = functions::get_files(&d);
            match files {
                Some(ref f) => index = functions::get_index(&arguments, &f),
                None => index = None
            }
        },
        None => {files = None; index = None;}
    }

    match files {
        Some(ref f) => {
            match index {
                Some(i) => println!("{:?}",f[i]),
                None => println!("Directory specified")
            }
        }
        None => println!("Nothing specified")
    }
    
    // 4. Set up GUI application
    let app = gui::NeptuneApp::setup(dir,files,index);
    let native_options = eframe::NativeOptions::default();

    // 5. Hand off management of the app to egui
    eframe::run_native(Box::new(app), native_options);
}