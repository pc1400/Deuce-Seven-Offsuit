extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate embedded_graphics;
extern crate find_folder;


use glutin_window::GlutinWindow as Window;
use graphics::triangulation::rect_tri_list_xy;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings,};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::path::Path;


use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Rectangle, RoundedRectangle, PrimitiveStyle, PrimitiveStyleBuilder},
};


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 0.75, 0.0, 1.0];
        const DARK_GREEN: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
        const RED: [f32; 4] = [0.5, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GRAY: [f32; 4] = [0.05, 0.05, 0.05, 1.0];
        

        let table = rectangle::centered(rectangle::rectangle_by_corners(25.0, 25.0, 375.0, 225.0));

        let outer_ring = rectangle::centered(rectangle::rectangle_by_corners(25.0, 25.0, 385.0, 235.0));
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        let carpet   = Image::new().rect(rectangle::square(0.0, 0.0, 300.0));
        let carpet2   = Image::new().rect(rectangle::square(300.0, 0.0, 300.0));
        let carpet3   = Image::new().rect(rectangle::square(600.0, 0.0, 300.0));
        let carpet4   = Image::new().rect(rectangle::square(900.0, 0.0, 300.0));
        let carpet5   = Image::new().rect(rectangle::square(0.0, 300.0, 300.0));
        let carpet6  = Image::new().rect(rectangle::square(300.0, 300.0, 300.0));
        let carpet7 = Image::new().rect(rectangle::square(600.0, 300.0, 300.0));
        let carpet8= Image::new().rect(rectangle::square(900.0, 300.0, 300.0));

        //A texture to use with the idmage
        let texture = Texture::from_path(Path::new("Seamless hotel casino carpet texture.jpeg"), &TextureSettings::new()).unwrap();

        let card1   = Image::new().rect(rectangle::square(220.0, 225.0, 100.0));
        let card2   = Image::new().rect(rectangle::square(335.0, 225.0, 100.0));
        let card3   = Image::new().rect(rectangle::square(450.0, 225.0, 100.0));
        let card4   = Image::new().rect(rectangle::square(565.0, 225.0, 100.0));
        let card5   = Image::new().rect(rectangle::square(680.0, 225.0, 100.0));

        let card7   = Image::new().rect(rectangle::square(380.0, 410.0, 100.0));
        let card8   = Image::new().rect(rectangle::square(485.0, 410.0, 100.0));

        let card9   = Image::new().rect(rectangle::square(40.0, 50.0, 100.0));
        let card10   = Image::new().rect(rectangle::square(145.0, 50.0, 100.0));


        let card_texture = Texture::from_path(Path::new("Playing Cards/PNG-cards-1.3/8cEbeEMLi.png"), &TextureSettings::new()).unwrap();

        let text_box = rectangle::centered([10.0, 270.0, 105.0, 20.0]);
        let text_box2 = rectangle::centered([-332.0, -90.0, -102.5, 20.0]);


        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(RED, gl);

            let transform = c
                .transform
                .trans(x, y)
                .trans(-25.0, -25.0);



            // Draw a box rotating around the middle of the screen.
            carpet.draw(&texture, &DrawState::new_alpha(), c.transform, gl);
            carpet2.draw(&texture, &DrawState::new_alpha(), c.transform, gl);
            carpet3.draw(&texture, &DrawState::new_alpha(), c.transform, gl);
            carpet4.draw(&texture, &DrawState::new_alpha(), c.transform, gl);
            carpet5.draw(&texture, &DrawState::new_alpha(), c.transform, gl);
            carpet6.draw(&texture, &DrawState::new_alpha(), c.transform, gl);
            carpet7.draw(&texture, &DrawState::new_alpha(), c.transform, gl);
            carpet8.draw(&texture, &DrawState::new_alpha(), c.transform, gl);

            ellipse(DARK_GREEN, table, transform, gl);
            circle_arc(GREEN, 3.0, 0.0, 100.0, table, transform, gl);
            circle_arc(BLACK, 7.0, 0.0, 100.0, outer_ring, transform, gl);

            card1.draw(&card_texture, &DrawState::new_alpha(), c.transform, gl);
            card2.draw(&card_texture, &DrawState::new_alpha(), c.transform, gl);
            card3.draw(&card_texture, &DrawState::new_alpha(), c.transform, gl);
            card4.draw(&card_texture, &DrawState::new_alpha(), c.transform, gl);
            card5.draw(&card_texture, &DrawState::new_alpha(), c.transform, gl);

            card7.draw(&card_texture, &DrawState::new_alpha(), c.transform, gl);
            card8.draw(&card_texture, &DrawState::new_alpha(), c.transform, gl);

            rectangle(GRAY, text_box, transform, gl);

            card9.draw(&card_texture, &DrawState::new_alpha(), c.transform, gl);
            card10.draw(&card_texture, &DrawState::new_alpha(), c.transform, gl);

            rectangle(GRAY, text_box2, transform, gl);

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Poker Table", [1000.0, 560.0])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
