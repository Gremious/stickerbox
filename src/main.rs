use std::{hash::Hash, ops::Deref, path::{Path, PathBuf}, rc::Rc, sync::{atomic::{AtomicU32, Ordering}, Arc}};
// use widgets as w;
use image::ImageEncoder;
use pariter::IteratorExt;
use slint::{ Model, ModelRc, SharedString, VecModel};

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


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PathEntry {
	Dir(PathBuf),
	File(PathBuf),
}

impl Deref for PathEntry {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
		match self {
			PathEntry::Dir(x) | PathEntry::File(x) => x
		}
    }
}


// #[tokio::main]
fn main() {
    let main_window = MainWindow::new().unwrap();

	// let window_weak = main_window.as_weak();

	let imgs_path = PathBuf::from("./resources/many-big-image");

	let entries = std::fs::read_dir(imgs_path).unwrap()
		.into_iter()
		.filter_map(|e| e.ok())
		.filter_map(|e| {
			let path = e.path();
			if path.is_dir() {
				Some(PathEntry::Dir(path))
			} else {
				match path.extension().and_then(|ext| ext.to_str()) {
					Some("jpg") | Some("png") | Some("webp") |  Some("gif") | Some("bmp") => {
						Some(PathEntry::File(path))
					},
					_ => None,
				}
			}
		});

	let buffers = entries
		.filter(|path| {
			// println!("path: {path:?}");
			!matches!(path, PathEntry::Dir(_))
		})
		.parallel_map(|path| {
			let image = image::load_from_memory(&std::fs::read(&*path).unwrap()).unwrap();
			let resized_img = image.resize_exact(128, 128, image::imageops::FilterType::Triangle);
			drop(image);
			let rgba8_image = resized_img.to_rgba8();

			let buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(
				rgba8_image.as_raw(),
				rgba8_image.width(),
				rgba8_image.height(),
			);

			buffer
		});

		let images = buffers
			.map(|buffer| slint::Image::from_rgba8(buffer))

		.collect::<VecModel<slint::Image>>();
	let count = images.row_count();
	let height = (count * 128) + ((count - 1) * 8);

	main_window.set_scroll_height(height as f32);
	main_window.set_images(Rc::new(images).into());

	// let path = PathBuf::from("./resources/GGST_Potemkin_Potemkin_Buster_Startup.png");
	// main_window.set_test_image(slint::Image::load_from_path(&path).unwrap());

	// main_window.on_click_button_cb(move |event| {
		// // println!("Evvent: {event:#?}");
		// if event.button == slint::platform::PointerEventButton::Left {
			// if event.kind.to_string() == "down" {
				// println!("down");
				// window_weak.unwrap().set_btn_clicked(true)
			// } else if event.kind.to_string() == "up" {
				// println!("up");
				// window_weak.unwrap().set_btn_clicked(false);
			// }
		// }
	// });

    main_window.run().unwrap();
}
