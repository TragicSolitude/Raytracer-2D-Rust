#[macro_use]
extern crate bitflags;

mod app;

use app::App;
use glutin_window::{OpenGL, GlutinWindow};
use piston::window::WindowSettings;
use opengl_graphics::GlGraphics;
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderEvent, UpdateEvent, MouseCursorEvent, PressEvent, ReleaseEvent};

const OPENGL_VERSION: OpenGL = OpenGL::V3_3;

fn main() {
    let mut window: GlutinWindow = WindowSettings::new("2D Raytracer", [750, 750])
        .graphics_api(OPENGL_VERSION)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(OPENGL_VERSION));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.update_mouse(&args);
        }

        if let Some(args) = e.press_args() {
            app.enable_key(args);
        }

        if let Some(args) = e.release_args() {
            app.disable_key(args);
        }
    }
}