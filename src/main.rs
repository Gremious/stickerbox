use floem::{prelude::*, window::WindowConfig, Application};

fn main() {
	// let config = WindowConfig::default().undecorated(true).undecorated_shadow(true);
    Application::new().window(move |_| counter_view(), None).run()
}

fn counter_view() -> impl IntoView {
    let mut counter = RwSignal::new(0);

    h_stack((
        button("Increment").action(move || counter += 1),
        label(move || format!("Value: {counter}")),
        button("Decrement").action(move || counter -= 1),
    ))
    .style(|s| s.size_full().items_center().justify_center().gap(10))
}
