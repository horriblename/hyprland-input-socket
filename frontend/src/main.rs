use gtk4::{glib, prelude::*, cairo::{Region, RectangleInt}, gdk::DisplayManager};
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

    // fn new_circle() -> cairo::path
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

    let style = window.style_context();
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data("* {
        background-color: transparent;
        border: solid 5px red; 
    }");
    style.add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    window.set_title(Some("First GTK Program"));
    window.set_default_size(350, 70);

    let canvas = build_canvas();

    window.set_child(Some(&canvas));

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

fn build_canvas() -> gtk4::DrawingArea {
    let canvas = gtk4::DrawingArea::new();

    let style = canvas.style_context();
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data("* {background-color: transparent;}");
    style.add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    canvas.set_draw_func(|_canvas, cairo_ctx: &gtk4::cairo::Context, _, _| {
        cairo_ctx.arc(10.0, 10.0, 10.0, 0.0, 2.0 * std::f64::consts::PI);
        cairo_ctx.set_source_rgba(0.9, 0.9, 0.9, 0.9);
        _ = cairo_ctx.fill();
    });

    canvas
}
