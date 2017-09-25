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

pub type Segment = [Point; 2];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    vor_pts: Vec<Point>,
    lines: Vec<Segment>,
    faces: Vec<([f32; 4], Vec<Point>)>,
    box_shift: f64,
    segs: Vec<Segment>,
    int_pts: Vec<Point>,
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
        let my_segs = self.segs.clone();
        let int_pts = self.int_pts.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let ctrans = c.transform.trans(box_shift, box_shift);

            // for this_seg in my_segs {
            //     line(BLACK, 1.0, [this_seg[0].x(), this_seg[0].y(), this_seg[1].x(), this_seg[1].y()], ctrans, gl);
            // }

            for pt in vor_pts {
	            let transform = ctrans.trans(pt.x(), pt.y())
	                                       .trans(-DOTSIZE/2., -DOTSIZE/2.);
	            ellipse(GREEN, square, transform, gl);
	        }

            // for pt in int_pts {
            //     let transform = ctrans.trans(pt.x(), pt.y())
            //                                .trans(-DOTSIZE/2., -DOTSIZE/2.);
            //     ellipse(RED, square, transform, gl);
            // }

            for this_line in lines {
                line(BLACK, 1.0, [this_line[0].x(), this_line[0].y(), this_line[1].x(), this_line[1].y()], ctrans, gl);
            }

            // for (this_color, this_face) in faces {
            //     let mut poly_pts = vec![];
            //     for pt in this_face {
            //         poly_pts.push([pt.x(), pt.y()]);
            //     }
            //     polygon(this_color, poly_pts.as_slice(), ctrans, gl);
            // }

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
    const LINE_BOX: f64 = 150.0;


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

    // Generate random segments
    let mut a_pts = vec![];
    for _ in 0..NUM_POINTS {
    	a_pts.push(rand::random::<Point>() * BOX_SIZE)
    }

    let mut b_pts = vec![];
    for _ in 0..NUM_POINTS {
        b_pts.push(rand::random::<Point>() * LINE_BOX - Point::new(LINE_BOX / 2., LINE_BOX / 2.));
    }

    let mut my_segs = vec![];
    for i in 0..NUM_POINTS {
        my_segs.push([a_pts[i as usize], a_pts[i as usize] + b_pts[i as usize]]);
    }

    // // Find intersections of random segments
    // let intersections = all_intersections(my_segs.clone());
    // let mut my_int_pts = vec![];
    // for intersection in intersections {
    //     my_int_pts.push(intersection.0);
    // }
    // println!("Found {} intersections.", my_int_pts.len());

    // let my_pts = vec![Point::new(139., 68.),
    //                     Point::new(127., 106.),
    //                     Point::new(87., 77.),
    //                     Point::new(71., 42.),
    //                     Point::new(46., 114.)];


    debug!("Computing Voronoi Diagram of {:?}", vor_pts);
    let sw = Stopwatch::start_new();
    let mut voronoi = voronoi(vor_pts.clone(), BOX_SIZE);
    info!("Voronoi of {} pts took {}ms", NUM_POINTS, sw.elapsed_ms());

    debug!("\n\n");
    debug!("Voronoi:\n{}", voronoi);

    // remove parts outside box
    // for vert in 0..voronoi.vertices.len() {
    //     let this_pt = voronoi.vertices[vert].coordinates;
    //     if outside_bb(this_pt, BOX_SIZE) {
    //         voronoi.remove_vertex(vert);
    //     }
    // }

    let sw_lines = Stopwatch::start_new();
    let lines = make_line_segments(&voronoi);
    info!("Making line segments took {}ms", sw_lines.elapsed_ms());
    debug!("Lines:\n{:?}", lines);

    let sw_faces = Stopwatch::start_new();
    add_faces(&mut voronoi);
    info!("Making faces took {}ms", sw_faces.elapsed_ms());

    let sw_polys = Stopwatch::start_new();
    let faces = make_polygons(&voronoi);
    info!("Making polygons took {}ms", sw_polys.elapsed_ms());

    let mut colored_faces = vec![];
    for face in faces {
        let this_color = [rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0];
        colored_faces.push((this_color, face));
    }

    let mut int_pts = vec![];
    for vertex in voronoi.vertices {
        if vertex.alive {
            int_pts.push(vertex.coordinates);
        }
    }

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        vor_pts: vor_pts,
        lines: lines,
        faces: colored_faces,
        box_shift: ((WINDOW_SIZE as f64) - BOX_SIZE) / 2.,
        segs: my_segs,
        // bb_segs: bb_segs,
        int_pts: int_pts,
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