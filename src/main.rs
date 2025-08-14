use std::{cell::RefCell, hash::Hash, ops::Deref, path::{Path, PathBuf}, rc::Rc, sync::{atomic::{AtomicU32, Ordering}, Arc, LazyLock, Mutex, RwLock}};
// use widgets as w;
use image::ImageEncoder;
use pariter::IteratorExt;
use slint::{ winit_030::{WinitWindowAccessor, WinitWindowEventResult}, Model, ModelRc, SharedString, VecModel};
use winit::raw_window_handle::HasWindowHandle;

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

static CURR_TAB: LazyLock<RwLock<SettingsTab>> = LazyLock::new(|| RwLock::new(SettingsTab::General));

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

fn main() {
	slint::BackendSelector::new().backend_name(String::from("winit")).select().unwrap();

    let window = MainWindow::new().unwrap();
	let imgs_path = PathBuf::from("./resources/");

	let window_handle = window.as_weak();

	// window.set_directory_from_search(SharedString::from(imgs_path.to_string_lossy().to_string()));
	// window.global::<SearchShit>().set_location(SharedString::from(imgs_path.to_string_lossy().to_string()));

    window.on_show_secondary_window(|| {
		let secondary_window = SearchPopUp::new().unwrap();
        secondary_window.show().unwrap();
    });

	window.invoke_show_secondary_window();

    window.run().unwrap();
}
