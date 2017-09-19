extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate voronoi_gen;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use voronoi_gen::{Point, voronoi};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    points: Vec<Point> 
}

#[allow(unused_variables)]
impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        const DOTSIZE: f64 = 5.0;

        let square = rectangle::square(0.0, 0.0, DOTSIZE);
        
        let points = self.points.clone();

        let voronoi = voronoi(points.clone());

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            for pt in points {

	            let transform = c.transform.trans(pt.x(), pt.y())
	                                       .trans(-DOTSIZE/2., -DOTSIZE/2.);

	            ellipse(BLACK, square, transform, gl);
	        }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
       
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    const WINDOW_SIZE: u32 = 400;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "voronoi-gen",
            [WINDOW_SIZE, WINDOW_SIZE]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    const NUM_POINTS: u32 = 100;

    let mut random_pts = vec![];
    for _ in 0..NUM_POINTS {
    	random_pts.push(rand::random::<Point>() * (WINDOW_SIZE as f64))
    }

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        points: random_pts,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}