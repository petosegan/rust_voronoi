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
use voronoi_gen::{Point, voronoi, make_line_segments, make_polygons, add_faces, lloyd_relaxation};
use stopwatch::{Stopwatch};

pub type Segment = [Point; 2];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    vor_pts: Vec<Point>,
    lines: Vec<Segment>,
    faces: Vec<([f32; 4], Vec<Point>)>,
    box_shift: f64,
}

#[allow(unused_variables)]
impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        // const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        // const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        const DOTSIZE: f64 = 3.0;

        let square = rectangle::square(0.0, 0.0, DOTSIZE);
        
        let vor_pts = self.vor_pts.clone();
        let lines = self.lines.clone();
        let faces = self.faces.clone();
        let box_shift = self.box_shift;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let ctrans = c.transform.trans(box_shift, box_shift);

            for pt in vor_pts {
	            let transform = ctrans.trans(pt.x(), pt.y())
	                                       .trans(-DOTSIZE/2., -DOTSIZE/2.);
	            ellipse(GREEN, square, transform, gl);
	        }

            for (this_color, this_face) in faces {
                let mut poly_pts = vec![];
                for pt in this_face {
                    poly_pts.push([pt.x(), pt.y()]);
                }
                polygon(this_color, poly_pts.as_slice(), ctrans, gl);
            }

            for this_line in lines {
                line(BLACK, 1.0, [this_line[0].x(), this_line[0].y(), this_line[1].x(), this_line[1].y()], ctrans, gl);
            }

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
    const BOX_SIZE: f64 = 780.0;
    const NUM_POINTS: u32 = 3000;
    const NUM_LLOYD: usize = 2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "voronoi-gen",
            [WINDOW_SIZE, WINDOW_SIZE]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Points for Voronoi Diagram
    let mut vor_pts = vec![];
    for _ in 0..NUM_POINTS {
        vor_pts.push(rand::random::<Point>() * BOX_SIZE)
    }

    let mut lloyd = vor_pts;
    for _ in 0..NUM_LLOYD {
        lloyd = lloyd_relaxation(lloyd, BOX_SIZE);
    }
    let mut voronoi = voronoi(lloyd.clone(), BOX_SIZE);
    debug!("Voronoi result: {}", voronoi);

    let sw_lines = Stopwatch::start_new();
    let lines = make_line_segments(&voronoi);
    info!("Making line segments took {}ms", sw_lines.elapsed_ms());
    // debug!("Lines:\n{:?}", lines);

    let sw_faces = Stopwatch::start_new();
    add_faces(&mut voronoi);
    info!("Making faces took {}ms", sw_faces.elapsed_ms());

    let sw_polys = Stopwatch::start_new();
    let mut faces = make_polygons(&voronoi);
    info!("Making polygons took {}ms", sw_polys.elapsed_ms());

    // remove the outer face
    faces.sort_by(|a, b| a.len().cmp(&b.len()));
    while faces.len() > lloyd.len() { faces.pop(); }

    let mut faces_disp = String::new();
    for face in &faces {
        for pt in face {
            faces_disp.push_str(format!("{}, ", pt).as_str());
        }
        faces_disp.push_str("\n");
    }
    debug!("Faces:\n{}", faces_disp);

    let mut colored_faces = vec![];
    for face in faces {
        let this_color = [rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0];
        colored_faces.push((this_color, face));
    }

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        vor_pts: lloyd,
        lines: lines,
        faces: colored_faces,
        box_shift: ((WINDOW_SIZE as f64) - BOX_SIZE) / 2.,
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