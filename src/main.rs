mod widgets;
mod button;

use std::{hash::Hash, path::PathBuf, sync::{atomic::{AtomicU32, Ordering}, Arc}};

//TODO:
// https://docs.rs/pariter/latest/pariter/

// use widgets as w;
use floem::{event::{Event, EventListener, EventPropagation}, peniko::{self, Blob}, prelude::*, style::{Style, StyleClassRef}, style_class, window::WindowConfig, Application, ViewId};
use image::ImageEncoder;

// Minimize to tray
// Settings tab
// - input folders to scan for images
// - rename things on disk, set additional text options
// - do i even care about resizing, litary doesn't i don't think and it's fine
// - listary has settings in the tray menu, which is a diff window entirely, seperate from the "app"
// might not be bad idea actually

fn main() {
	// let config = WindowConfig::default()
		// // .undecorated(true)
		// // .undecorated_shadow(true)
        // .apply_default_theme(false);
	// // init_theme();
    Application::new()
		.window(move |_| main_view(), None).run()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PathEntry {
	Dir(PathBuf),
	File(PathBuf),
}


fn main_view() -> impl IntoView {
	floem::action::inspect();
    let search_text = create_rw_signal(String::new());
	let button_clicked = RwSignal::new(false);

	// let entries = std::fs::read_dir("./resources/many-big-image").unwrap()
		// .into_iter()
		// .filter_map(|e| e.ok())
		// .filter_map(|e| {
			// let path = e.path();
			// if path.is_dir() {
				// Some(PathEntry::Dir(path))
			// } else {
				// match path.extension().and_then(|ext| ext.to_str()) {
					// Some("jpg") | Some("png") | Some("webp") |  Some("gif") | Some("bmp") => {
						// Some(PathEntry::File(path))
					// },
					// _ => None,
				// }
			// }
		// })
		// .map(|path_entry| {
			// println!("{path_entry:?}");
			// match path_entry {
				// PathEntry::Dir(path_entry) => {
					// v_stack((
							// label(move || format!("{}", path_entry.display())).style(|s| s.height(20.0)),
					// ))
				// },
				// PathEntry::File(path_entry) => {
					// let path = path_entry.clone();
					// let image = image::ImageReader::open(path).unwrap().decode().unwrap();
					// let image = image.resize(64, 64, image::imageops::FilterType::Triangle);
					// let image_byes = image.as_bytes().to_vec();

					// v_stack((
							// img(move || image_byes.clone())
								// .style(|s| s.width(64.).height(64.)),
							// // label(move || format!("{}", path_entry.display())).style(|s| s.height(20.0))
					// ))
				// },

			// }
		// })
		// .collect::<Vec<Stack>>();

	// TODO: Make your own image object that doesn't need to read and re-encode and then decode again
	// OR BETTER YET: PR
	// Our pr will close https://github.com/lapce/floem/pull/605
	// someone tried it but got lazy with ci checks
	let path = "./resources/GGST_Potemkin_Potemkin_Buster_Startup.png";
	let image = image::ImageReader::open(path).unwrap().decode().unwrap();
	let image = image.resize(64, 64, image::imageops::FilterType::Triangle);

	// let image_bytes = image.into_bytes().len();
	// let real_bytes = std::fs::read(path).unwrap().len();
    // let data = image.into_rgba8().into_vec();
	// assert_eq!(image_bytes, real_bytes);

	let mut png_buf = Vec::new();
	let width = image.width();
	let height = image.height();
	let encoder = image::codecs::png::PngEncoder::new(&mut png_buf);
	encoder.write_image(image.clone().as_bytes(), width, height, image::ExtendedColorType::Rgba8).unwrap();

	// println!("{} vs png {}", real_bytes, png_buf.len());

	// let image = image.resize(64, 64, image::imageops::FilterType::Triangle);
	// let image = image
	// let image = image::load_from_memory(&image).unwrap();
	// image.save_with_format("./out.png", image::ImageFormat::Png).unwrap();

	v_stack((
		h_stack((
			img(move || {
				// let path = "./resources/GGST_Potemkin_Potemkin_Buster_Startup.png";
				// std::fs::read(path).unwrap()
				png_buf.clone()
			}),
				// list(entries),
			// virtual_list(
				// || { entries },
				// move |item| item.clone(),
				// move |item| {
					// println!("{item:?}");
					// match item {
						// PathEntry::Dir(path_entry) => {
							// v_stack((
								// label(move || format!("{}", path_entry.display())).style(|s| s.height(20.0)),
							// ))
						// },
						// PathEntry::File(path_entry) => {
							// let path = path_entry.clone();

							// v_stack((
								// img(move || std::fs::read(path.clone()).unwrap())
									// .style(|s| s.width(64.).height(64.)),
								// label(move || format!("{}", path_entry.display())).style(|s| s.height(20.0))
							// ))
						// },

					// }
				// },
			// ).style(|s| s.width_pct(50.).height_full().border(1)),
		)).style(|s| {
			s.min_size_full().items_center().justify_center()
		}),
	)).style(|s| s.size_full().justify_center().items_center())
}
