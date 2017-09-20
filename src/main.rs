#[macro_use] extern crate log;
extern crate env_logger;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate voronoi_gen;
extern crate rand;
extern crate stopwatch;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use voronoi_gen::{Point, voronoi, make_line_segments};
use stopwatch::{Stopwatch};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    points: Vec<Point>,
    lines: Vec<(Point, Point)> 
}

#[allow(unused_variables)]
impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        const DOTSIZE: f64 = 5.0;

        let square = rectangle::square(0.0, 0.0, DOTSIZE);
        
        let points = self.points.clone();
        let lines = self.lines.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            for pt in points {

	            let transform = c.transform.trans(pt.x(), pt.y())
	                                       .trans(-DOTSIZE/2., -DOTSIZE/2.);

	            ellipse(BLACK, square, transform, gl);
	        }

            for this_line in lines {
                let (p1, p2) = this_line;

                line(RED, 1.0, [p1.x(), p1.y(), p2.x(), p2.y()], c.transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
       
    }
}

#[allow(unused_must_use)]
fn main() {
    env_logger::init();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    const WINDOW_SIZE: u32 = 800;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "voronoi-gen",
            [WINDOW_SIZE, WINDOW_SIZE]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    const NUM_POINTS: u32 = 20;
    let mut my_pts = vec![];
    for _ in 0..NUM_POINTS {
    	my_pts.push(rand::random::<Point>() * (WINDOW_SIZE as f64))
    }

    // let my_pts = vec![Point::new(139., 68.),
    //                     Point::new(127., 106.),
    //                     Point::new(87., 77.),
    //                     Point::new(71., 42.),
    //                     Point::new(46., 114.)];


    let vor_pts = my_pts.clone();
    trace!("Computing Voronoi Diagram of {:?}", my_pts);
    let sw = Stopwatch::start_new();
    let voronoi = voronoi(vor_pts);
    println!("Voronoi of {} pts took {}ms", NUM_POINTS, sw.elapsed_ms());
    trace!("\n\n");
    trace!("Voronoi:\n{}", voronoi);

    let sw_lines = Stopwatch::start_new();
    let lines = make_line_segments(&voronoi);
    println!("Making line segments took {}ms", sw_lines.elapsed_ms());
    trace!("Lines:\n{:?}", lines);

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        points: my_pts,
        lines: lines
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