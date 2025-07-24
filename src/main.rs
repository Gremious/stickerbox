use std::{hash::Hash, path::PathBuf, sync::{atomic::{AtomicU32, Ordering}, Arc}};
// use widgets as w;
use image::ImageEncoder;
use slint::SharedString;

slint::include_modules!();

//TODO:
// https://docs.rs/pariter/latest/pariter/

// Minimize to tray
// Settings tab
// - input folders to scan for images
// - rename things on disk, set additional text options
// - do i even care about resizing, litary doesn't i don't think and it's fine
// - listary has settings in the tray menu, which is a diff window entirely, seperate from the "app"
// might not be bad idea actually

fn main() {
    let main_window = MainWindow::new().unwrap();

	let window_weak = main_window.as_weak();

	main_window.on_click_button_cb(move |event| {
		// println!("Evvent: {event:#?}");
		if event.button == slint::platform::PointerEventButton::Left {
			if event.kind.to_string() == "down" {
				println!("down");
				window_weak.unwrap().set_btn_clicked(true)
			} else if event.kind.to_string() == "up" {
				println!("up");
				window_weak.unwrap().set_btn_clicked(false);
			}
		}
	});

    main_window.run().unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PathEntry {
	Dir(PathBuf),
	File(PathBuf),
}
