// Description: file used to set up and manage the program window. 

use clap::ArgMatches;
use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::gdk::Display;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::{Application, ApplicationWindow, Button, CssProvider, Grid, Picture, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION};

pub fn setup() -> Application {
    let app = Application::builder()
        .application_id("org.andromeda.neptune")
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();
    app
}

pub fn manage_window(app: &Application, args: ArgMatches<'static>, supported_formats: Vec<&'static str>) {
    app.connect_activate(move |app| {
        // 0. Setup application CSS
        let provider = CssProvider::new();
        provider.load_from_data(include_bytes!("../resources/css/style.css"));
        StyleContext::add_provider_for_display(
            &Display::default().expect("Error initializing gtk css provider."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION
        );

        // 1. Setup application window (Read exif if image specified, get window dims)
        // Main application window
        let default_width = 960; // pixels
        let default_height = 640; // pixels

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(default_width) // Set aspect ratio to image
            .default_height(default_height) 
            .title("Neptune")
            .build();
        let grid = Grid::builder()
            .column_homogeneous(false)
            .row_homogeneous(false)
            .build();
        window.set_child(Some(&grid));

        // If image is provided in argument, open in ImageWidget. Otherwise, create button to open an image. 
        // GetImageData function - used to provide image data and EXIF data?
        // To resize, increase image size and set image center to where cursor is?
        // 
        // 2. Open image / create button
        // If we include a file in our argument, and that file is in our supported formats, open it. 
        if args.is_present("INPUT") && supported_formats.iter().any(|&f| args.value_of("INPUT").unwrap().contains(f)) {
            let path = args.value_of("INPUT").unwrap();
            let img = open_image(path, &window, default_width, default_height);
            grid.attach(&img,0,0,35,35);
        }
        // Otherwise, create a button to open an image. 
        else {
            let open_button = Button::with_label("Open image");
            open_button.connect_clicked(|_| {
                eprintln!("Clicked");
            });
            grid.attach(&open_button,15,15,4,4); // Does not center. TODO. 
        }

        // 3. Set up buttons if other files in directory
        let mut b1 = [0;3];
        let mut b2 = [0;3];
        let right_button = Button::with_label('\u{2192}'.encode_utf8(&mut b1));
        let left_button = Button::with_label('\u{2190}'.encode_utf8(&mut b2));

        right_button.add_css_class("right_button");
        left_button.add_css_class("left_button");

        right_button.set_margin_end(20);
        left_button.set_margin_start(20);

        grid.attach(&right_button,34,17,1,1);
        grid.attach(&left_button,0,17,1,1);

        grid.insert_row(3);

        // 4. Display window
        // Display the window
        window.show();

        // 5. Set up event controller and wait for input (zoom, arrows, clicks, etc).

    });

    app.run_with_args(&["0"]);
}

fn open_image(path: &str, window: &ApplicationWindow, default_width: i32, default_height: i32) -> Picture {
    let pb = Pixbuf::from_file(path).unwrap();
    // Resize window for image. Create function for this. 
    if pb.height() > pb.width() {
        let new_width = (pb.width() as f32 /pb.height() as f32)*default_height as f32;
        window.set_default_width(new_width.round() as i32);
        window.set_default_height(default_height);
    } else if pb.height() < pb.width() {
        let new_height = (pb.height() as f32/pb.width() as f32)*default_width as f32;
        window.set_default_height(new_height.round() as i32);
        window.set_default_width(default_width);
    } else if pb.height() == pb.width() {
        window.set_default_size(default_height,default_height); // arbitary square
    }
    let img = Picture::for_pixbuf(Some(&pb));
    img
}