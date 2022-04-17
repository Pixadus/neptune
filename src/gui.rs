// Description: file used to set up and manage the program window. 

use std::path::PathBuf;
use rfd::FileDialog;
use std::ffi::OsStr;
use eframe::{egui,epi};

pub struct NeptuneApp {
    // Current directory
    directory: Option<PathBuf>,

    // Vector containing valid files in directory
    images: Option<Vec<PathBuf>>,

    // Current index of images
    im_index: Option<usize>,
}

// Set up app using user-supplied variables
impl NeptuneApp {
    pub fn setup(dir: Option<PathBuf>, files: Option<Vec<PathBuf>>, index: Option<usize>) -> Self {
        Self {
            directory: dir,
            images: files,
            im_index: index
        }
    }
}

impl epi::App for NeptuneApp {
    fn name(&self) -> &str {
        "Neptune"
    }

    // Initial application setup
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {}

        /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { directory, images, im_index } = self;

        // Menu panel
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // This is where we store our menu bar
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // Open file dialogue
                        let opened_file = FileDialog::new()
                            .set_directory(
                                match directory {
                                    Some(ref d) => d.as_os_str(),
                                    None => OsStr::new("/")
                                }
                            )
                            .pick_file();
                        match opened_file {
                            Some(f) => *images = Some(vec!(f)),
                            None => {}
                        }
                    };
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    };
                });
            });
        });
        println!("{:?}", self.images);

    }
}