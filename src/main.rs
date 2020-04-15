#[macro_use]
extern crate bitflags;

mod app;

use app::App;
use piston_window::PistonWindow;
use piston_window::WindowSettings;
use piston_window::OpenGL;
use piston_window::UpdateEvent;
use piston_window::ReleaseEvent;
use piston_window::PressEvent;
// use piston_window::MouseCursorEvent;

const OPENGL_VERSION: OpenGL = OpenGL::V4_5;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("2D Raytracer", [750, 750])
            .graphics_api(OPENGL_VERSION)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut app = App::new();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |context, gl, _device| app.render(context, gl));

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        // if let Some(args) = e.mouse_cursor_args() {
        //     app.update_mouse(&args);
        // }

        if let Some(args) = e.press_args() {
            app.enable_key(&args);
        }

        if let Some(args) = e.release_args() {
            app.disable_key(&args);
        }
    }
}