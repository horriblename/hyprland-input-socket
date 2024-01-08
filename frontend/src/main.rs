use gtk4::{glib, prelude::*, cairo::{Region, RectangleInt}, gdk::DisplayManager};
use gdk4_wayland::prelude::DisplayExt;
use gtk4_layer_shell::{LayerShell, Layer};
use std::{collections::HashMap, os::unix};

struct TouchPoint {
    id: u32,
    pos: (i64, i64),
}

struct TouchPoints{
    points: HashMap<u32, TouchPoint>,
}

impl TouchPoints {
    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
        }
    }
    pub fn update(&mut self, id: u32, pos: (i64, i64)) {
        self.points.insert(id, TouchPoint{id, pos});
    }

    pub fn remove(&mut self, id: u32) {
        self.points.remove(&id);
    }
}



fn main() -> glib::ExitCode {
    let application = gtk4::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);

    window.set_title(Some("First GTK Program"));
    window.set_default_size(350, 70);

    let button = gtk4::Button::with_label("Click me!");

    window.set_child(Some(&button));

    window.present();

    let maybe_display = DisplayManager::get().default_display();
    if maybe_display.is_some_and(|display| display.supports_input_shapes()) {
        let rect = RectangleInt::new(0, 0, 0, 0);
        let region = Region::create_rectangle(&rect);
        let surface = window.surface();
        surface.set_input_region(&region)
    } else {
        println!("display does not support input shape");
    }

}
