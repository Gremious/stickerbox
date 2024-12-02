use floem::{prelude::*, window::WindowConfig, Application};

// Minimize to tray
// Settings tab
// - input folders to scan for images
// - rename things on disk, set additional text options
// - do i even care about resizing, litary doesn't i don't think and it's fine
// - listary has settings in the tray menu, which is a diff window entirely, seperate from the "app"
// might not be bad idea actually

fn main() {
	let config = WindowConfig::default()
		// .undecorated(true)
		// .undecorated_shadow(true)
        .apply_default_theme(false);
	// init_theme();
    Application::new()
		.window(move |_| main_view(), None).run()
}

fn main_view() -> impl IntoView {
    let mut counter = RwSignal::new(0);

	h_stack((
		v_stack((
			button("Increment").action(move || counter += 1),
			label(move || format!("Value: {counter}")),
			button("Decrement").action(move || counter -= 1),
		))
		.style(|x| x
			.width_pct(25.)
			.height_full()
			.gap(8)
			.padding(8)
			.border_right(2)
			.border_color(Color::DARK_GRAY)
		),

		v_stack((
			scroll((
				img(|| std::fs::read("./resources/GGST_Potemkin_Potemkin_Buster_Startup.png").unwrap())
					.on_click(move |_| {
						println!("Clicked image");
						floem::event::EventPropagation::Stop
					}),
			)).style(|s| s.size_full()),
		))
		.style(|s| s.height_full().width_pct(75.).items_center().justify_center().background(Color::REBECCA_PURPLE))
	))
    .style(|s| s.size_full().items_center().justify_center())
}
