#![allow(dead_code)]

use gtk4::{glib, prelude::*, cairo::{Region, RectangleInt}, gdk::DisplayManager};
use gtk4_layer_shell::{LayerShell, Layer};
use std::{collections::HashMap, rc::Rc, cell::RefCell};

struct TouchPoint {
    id: u32,
    pos: (f64, f64),
}

struct TouchPoints{
    screen_size: (i64, i64),
    points: HashMap<u32, TouchPoint>,
}

impl TouchPoints {
    pub fn new(screen_size: (i64, i64)) -> Self {
        Self {
            screen_size,
            points: HashMap::new(),
        }
    }
    pub fn update(&mut self, id: u32, pos: (f64, f64)) {
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
    // let window = &application.windows()[0];

    // main_loop(state, window);

    application.run()
}

fn main_loop(state: Rc<RefCell<TouchPoints>>, window: &gtk4::ApplicationWindow) {
    use std::os::unix::net::UnixStream;
    use std::io::prelude::*;

    let stream = UnixStream::connect("/tmp/hypr/.input.sock").unwrap();
    let mut line = String::new();

    fn parse_and_move(state: &mut TouchPoints, args: &str) -> Option<()> {
        let mut args_iter = args.split(",");
        let id: u32 = args_iter.next()?.parse().ok()?;
        let x: f64 = args_iter.next()?.parse().ok()?;
        let y: f64 = args_iter.next()?.parse().ok()?;

        state.update(id, (x, y));
        Some(())
    }

    for c in stream.bytes() {
        let c = c.unwrap();
        if c != b'\n' {
            line.push(c as char);
            continue;
        }

        // handle event
        let mut line_iter = line.split(">>");
        let Some(event_name) = line_iter.next() else {continue;};
        let Some(args) = line_iter.next() else {continue;};

        match event_name {
            "touchDown" | "touchMove" => {parse_and_move(&mut state.borrow_mut(), args);},
            "touchUp" => {
                let Ok(id) = args.parse::<u32>() else { continue; };
                state.borrow_mut().remove(id);
            },
            _ => {},
        }
        println!("fingers: {}", state.borrow().points.len());

        window.queue_draw();
        gtk4::glib::MainContext::default().iteration(true);
        line.clear();
    }
}

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);
    let state = Rc::new(RefCell::new(TouchPoints::new((0, 0))));

    let Some(display) = DisplayManager::get().default_display() else {
        println!("no display found");
        return;
    };

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);

    let style = window.style_context();
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data("* {
        all: unset;
    }");
    style.add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    window.set_title(Some("First GTK Program"));

    if let Some(monitor) = display.monitors().item(0) {
        let default_size = monitor.dynamic_cast::<gtk4::gdk::Monitor>().unwrap().geometry();
        window.set_default_size(default_size.width(), default_size.height());
    } else {
        println!("no monitors found!");
        return;
    };

    let canvas = build_canvas(state.clone());

    window.set_child(Some(&canvas));

    window.present();

    let surface = window.surface();
    if display.supports_input_shapes() {
        let rect = RectangleInt::new(0, 0, 0, 0);
        let region = Region::create_rectangle(&rect);
        surface.set_input_region(&region)
    } else {
        println!("display does not support input shape");
    }

    if let Some(monitor) = display.monitor_at_surface(&surface) {
        let geometry = monitor.geometry();
        state.borrow_mut().screen_size = (geometry.width() as i64, geometry.height() as i64);
    }

    main_loop(state, &window);
}

fn build_canvas(state: Rc<RefCell<TouchPoints>>) -> gtk4::DrawingArea {
    let canvas = gtk4::DrawingArea::new();

    let style = canvas.style_context();
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data("* {background-color: transparent;}");
    style.add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    canvas.set_draw_func(move |_canvas, cairo_ctx: &gtk4::cairo::Context, _, _| {
        // let borrowed = state.borrow();
        let state = state.borrow();
        let screen_size = state.screen_size;
        for c in &state.points {
            let pos = c.1.pos;
            cairo_ctx.arc(
                pos.0 * screen_size.0 as f64,
                pos.1 * screen_size.0 as f64,
                10.0,
                0.0,
                2.0 * std::f64::consts::PI,
            );
            cairo_ctx.set_source_rgba(0.9, 0.9, 0.9, 0.9);
            _ = cairo_ctx.fill();
        }
    });

    canvas
}
