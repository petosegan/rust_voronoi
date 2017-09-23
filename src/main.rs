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
use voronoi_gen::{Point, voronoi, make_line_segments, make_polygons, add_faces, all_intersections};
use stopwatch::{Stopwatch};

pub type Segment = [Point; 2];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    points: Vec<Point>,
    lines: Vec<(Point, Point)>,
    faces: Vec<([f32; 4], Vec<Point>)>,
    box_shift: f64,
    box_side: f64,
    segs: Vec<Segment>
}

#[allow(unused_variables)]
impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        const DOTSIZE: f64 = 10.0;

        let square = rectangle::square(0.0, 0.0, DOTSIZE);
        
        let points = self.points.clone();
        let lines = self.lines.clone();
        let faces = self.faces.clone();
        let box_shift = self.box_shift;
        let box_side = self.box_side;
        let my_segs = self.segs.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);


            for this_seg in my_segs {
                line(BLACK, 1.0, [this_seg[0].x(), this_seg[0].y(), this_seg[1].x(), this_seg[1].y()], c.transform.trans(box_shift, box_shift), gl);
            }

            for pt in points {

	            let transform = c.transform.trans(pt.x(), pt.y())
	                                       .trans(-DOTSIZE/2., -DOTSIZE/2.)
                                           .trans(box_shift, box_shift);

	            ellipse(RED, square, transform, gl);
	        }

            // for this_line in lines {
            //     let (p1, p2) = this_line;

            //     line(RED, 1.0, [p1.x(), p1.y(), p2.x(), p2.y()], c.transform.trans(box_shift, box_shift), gl);
            // }

            

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
    const NUM_POINTS: u32 = 200;
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

    let mut my_pts = vec![];
    for _ in 0..NUM_POINTS {
    	my_pts.push(rand::random::<Point>() * BOX_SIZE)
    }

    let mut b_pts = vec![];
    for _ in 0..NUM_POINTS {
        b_pts.push(rand::random::<Point>() * LINE_BOX - Point::new(LINE_BOX / 2., LINE_BOX / 2.));
    }

    let mut my_segs = vec![];
    for i in 0..NUM_POINTS {
        my_segs.push([my_pts[i as usize], my_pts[i as usize] + b_pts[i as usize]]);
    }

    let intersections = all_intersections(my_segs.clone());
    let mut my_int_pts = vec![];
    for intersection in intersections {
        my_int_pts.push(intersection.0);
    }
    println!("Found {} intersections.", my_int_pts.len());

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
        points: my_int_pts,
        lines: lines,
        faces: colored_faces,
        box_shift: ((WINDOW_SIZE as f64) - BOX_SIZE) / 2.,
        box_side: BOX_SIZE,
        segs: my_segs
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