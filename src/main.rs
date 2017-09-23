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
use voronoi_gen::{Point, voronoi, make_line_segments, make_polygons, add_faces};
use stopwatch::{Stopwatch};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    points: Vec<Point>,
    lines: Vec<(Point, Point)>,
    faces: Vec<([f32; 4], Vec<Point>)>,
    box_shift: f64,
    box_side: f64,
}

#[allow(unused_variables)]
impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];


        const DOTSIZE: f64 = 5.0;

        let square = rectangle::square(0.0, 0.0, DOTSIZE);
        
        let points = self.points.clone();
        let lines = self.lines.clone();
        let faces = self.faces.clone();
        let box_shift = self.box_shift;
        let box_side = self.box_side;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            for pt in points {

	            let transform = c.transform.trans(pt.x(), pt.y())
	                                       .trans(-DOTSIZE/2., -DOTSIZE/2.)
                                           .trans(box_shift, box_shift);

	            ellipse(BLACK, square, transform, gl);
	        }

            for this_line in lines {
                let (p1, p2) = this_line;

                line(RED, 1.0, [p1.x(), p1.y(), p2.x(), p2.y()], c.transform.trans(box_shift, box_shift), gl);
            }

            // for (this_color, this_face) in faces {

            //     let mut poly_pts = vec![];
            //     for pt in this_face {
            //         poly_pts.push([pt.x(), pt.y()]);
            //     }
            //     polygon(this_color, poly_pts.as_slice(), c.transform.trans(box_shift, box_shift), gl);
            // }

            // draw bounding box
            line(BLUE, 1.0, [0., 0., box_side, 0.], c.transform.trans(box_shift, box_shift), gl);
            line(BLUE, 1.0, [0., 0., 0., box_side], c.transform.trans(box_shift, box_shift), gl);
            line(BLUE, 1.0, [box_side, box_side, box_side, 0.], c.transform.trans(box_shift, box_shift), gl);
            line(BLUE, 1.0, [box_side, box_side, 0., box_side], c.transform.trans(box_shift, box_shift), gl);


        });
    }

    fn update(&mut self, args: &UpdateArgs) {
       
    }
}

#[allow(unused_must_use)]
fn main() {
    let _ = env_logger::init();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    const WINDOW_SIZE: u32 = 800;
    const BOX_SIZE: f64 = 400.0;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "voronoi-gen",
            [WINDOW_SIZE, WINDOW_SIZE]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    const NUM_POINTS: u32 = 50;
    let mut my_pts = vec![];
    for _ in 0..NUM_POINTS {
    	my_pts.push(rand::random::<Point>() * BOX_SIZE)
    }

    // let my_pts = vec![Point::new(139., 68.),
    //                     Point::new(127., 106.),
    //                     Point::new(87., 77.),
    //                     Point::new(71., 42.),
    //                     Point::new(46., 114.)];


    let vor_pts = my_pts.clone();
    debug!("Computing Voronoi Diagram of {:?}", my_pts);
    let sw = Stopwatch::start_new();
    let mut voronoi = voronoi(vor_pts);
    info!("Voronoi of {} pts took {}ms", NUM_POINTS, sw.elapsed_ms());

    let sw_lines = Stopwatch::start_new();
    let lines = make_line_segments(&voronoi);
    info!("Making line segments took {}ms", sw_lines.elapsed_ms());
    debug!("Lines:\n{:?}", lines);

    let sw_faces = Stopwatch::start_new();
    add_faces(&mut voronoi);
    info!("Making faces took {}ms", sw_faces.elapsed_ms());

    debug!("\n\n");
    debug!("Voronoi:\n{}", voronoi);

    let sw_polys = Stopwatch::start_new();
    let faces = make_polygons(&voronoi);
    info!("Making polygons took {}ms", sw_polys.elapsed_ms());

    
    let mut colored_faces = vec![];
    for face in faces {
        let this_color = [rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0];
        colored_faces.push((this_color, face));
    }

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        points: my_pts,
        lines: lines,
        faces: colored_faces,
        box_shift: ((WINDOW_SIZE as f64) - BOX_SIZE) / 2.,
        box_side: BOX_SIZE
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