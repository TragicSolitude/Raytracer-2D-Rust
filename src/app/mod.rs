use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use std::f64::consts::PI;
use std::cmp::Ordering;
use graphics::math::triangle_face;
use graphics::draw_state::Blend::Lighter;

#[derive(Clone, Copy, Debug)]
struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    // constructor omitted

    // setIntersectsOutside method omitted

    pub fn distance_to(&self, other: &Point) -> f64 {
        // return Math.sqrt(Math.pow(other.x - this.x, 2) + Math.pow(other.y - this.y, 2));
        return ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt();
    }
}

// LightSource class omitted

#[derive(Clone)]
struct LightSource {
    pub pos: Point,
    pub visible: Vec<Point>,
    // color omitted
    // all omitted
}

impl LightSource {
    pub fn new() -> Self {
        LightSource {
            pos: Point { x: 0.0, y: 0.0 },
            visible: Vec::new()
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64) -> &mut Self {
        self.pos.x = x;
        self.pos.y = y;

        self
    }
}

struct Vector {
    pub dir: f64,
    pub mag: f64
}

impl Vector {
    // constructor omitted

    pub fn add(&self, other: &Vector) -> Self {
        // let dir = Math.atan(
        //     ((Math.sin(this.dir) + Math.sin(other.dir)) / 2) /
        //     ((Math.cos(this.dir) + Math.cos(other.dir)) / 2)
        // );
        let dir_component_0 = (self.dir.sin() + other.dir.sin()) / 2.0;
        let dir_component_1 = (self.dir.cos() + other.dir.cos()) / 2.0;
        let dir = (dir_component_0 / dir_component_1).atan();

        // let mag = (this.mag + other.mag) / 2;
        let mag = (self.mag + other.mag) / 2.0;

        // return new Vector(dir, mag);
        Vector { dir, mag }
    }
}

struct Segment {
    pub start: Point,
    pub end: Point,
    pub dir: f64,
    pub length: f64
}

impl Segment {
    // dir and length arguments omitted
    pub fn new(p1: Point, p2: Point) -> Self {
        // this.length = this.start.distanceTo(this.end);
        let length = p1.distance_to(&p2);
        let dir = (p2.y - p1.y).atan2(p2.x - p1.x);

        Segment { start: p1, end: p2, dir, length }
    }

    // moved Ray subclass to secondary constructor on Segment
    // omitted dir argument
    pub fn ray(p1: Point, dest: Point) -> Self {
        let dir = (dest.y - p1.y).atan2(dest.x - p1.x);
        // super(p1, new Point(p1.x + Math.cos(dir) * 2147483647, p1.y + Math.sin(dir) * 2147483647), dir);
        let end = Point {
            x: p1.x + dir.cos() * 2147483647.0,
            y: p1.y + dir.sin() * 2147483647.0
        };
        let length = p1.distance_to(&end);

        Segment { start: p1, end, dir, length }
    }
}

#[derive(Clone)]
struct Rectangle {
    pub pos: Point,
    pub width: f64,
    pub height: f64
    // color omitted
}

impl Rectangle {
    pub fn get_points(&self) -> [Point; 4] {
        [
            Point { x: self.pos.x, y: self.pos.y },
            Point { x: self.pos.x + self.width, y: self.pos.y },
            Point { x: self.pos.x + self.width, y: self.pos.y + self.height },
            Point { x: self.pos.x, y: self.pos.y + self.height}
        ]
    }

    pub fn get_segments(&self) -> [Segment; 4] {
        let points = self.get_points();

        return [
            Segment::new(points[0], points[1]),
            Segment::new(points[1], points[2]),
            Segment::new(points[2], points[3]),
            Segment::new(points[3], points[0])
        ]
    }

    pub fn center_point(&self) -> Point {
        Point {
            x: self.pos.x + (self.width / 2.0),
            y: self.pos.y + (self.height / 2.0)
        }
    }

    // Added for compatibility with piston library
    pub fn output(&self) -> graphics::types::Rectangle {
        [
            self.pos.x,
            self.pos.y,
            self.width,
            self.height
        ]
    }
}

fn calculate_line_intersect(p0_x: f64, p0_y: f64, p1_x: f64, p1_y: f64, p2_x: f64, p2_y: f64, p3_x: f64, p3_y: f64) -> Option<Point> {
    let s1_x = p1_x - p0_x;
    let s1_y = p1_y - p0_y;
    let s2_x = p3_x - p2_x;
    let s2_y = p3_y - p2_y;

    let s = (-s1_y * (p0_x - p2_x) + s1_x * (p0_y - p2_y)) / (-s2_x * s1_y + s1_x * s2_y);
    let t = ( s2_x * (p0_y - p2_y) - s2_y * (p0_x - p2_x)) / (-s2_x * s1_y + s1_x * s2_y);

    if s>= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
        let i_x = p0_x + (t * s1_x);
        let i_y = p0_y + (t * s1_y);

        return Some(Point { x: i_x, y: i_y });
    }

    None
}

fn get_line_intersect(s1: &Segment, s2: &Segment) -> Option<Point> {
    calculate_line_intersect(s1.start.x, s1.start.y, s1.end.x, s1.end.y, s2.start.x, s2.start.y, s2.end.x, s2.end.y)
}

pub struct App {
    gl: GlGraphics,
    lights: Vec<LightSource>,
    shapes: Vec<Rectangle>
}

impl App {
    pub fn new(gl: GlGraphics) -> Self {
        let mut lightsource = LightSource::new();
        lightsource.move_to(485.0, 485.0);

        let lights = vec![
            lightsource
        ];

        let shapes = vec![
            Rectangle { pos: Point { x: 0.0, y: 0.0 }, width: 750.0, height: 750.0 },
            Rectangle { pos: Point { x: 30.0, y: 30.0 }, width: 80.0, height: 80.0 },
            Rectangle { pos: Point { x: 400.0, y: 80.0 }, width: 60.0, height: 120.0 },
            Rectangle { pos: Point { x: 300.0, y: 550.0 }, width: 350.0, height: 50.0 }
        ];

        App {
            gl,
            lights,
            shapes
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const COLOR_BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const COLOR_SHAPES: [f32; 4] = [0.0, 128.0, 128.0, 1.0];
        const COLOR_LIGHT: [f32; 4] = [255.0, 255.0, 255.0, 0.2];

        let shapes = self.shapes.clone();
        let lights = self.lights.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(COLOR_BACKGROUND, gl);

            for shape in shapes.iter().skip(1) {
                rectangle(COLOR_SHAPES, shape.output(), c.transform, gl);
            }

            for light in lights.iter() {
                let mut visible = light.visible.iter().peekable();
                while let Some(intersect) = visible.next() {
                    let peek = visible.peek();

                    if let Some(peek) = visible.peek() {
                        let poly = [
                            [light.pos.x, light.pos.y],
                            [intersect.x, intersect.y],
                            [peek.x, peek.y],
                        ];
                        polygon(COLOR_LIGHT, &poly, c.transform, gl);
                    }
                }
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for light in self.lights.iter_mut() {
            let mut rays = Vec::with_capacity(self.shapes.len() * 4 * 3);

            for shape in self.shapes.iter() {
                for point in shape.get_points().iter() {
                    // Unused
                    // let dir = (light.pos.y - point.y).atan2(light.pos.x - point.x) + PI;

                    // rays.push(new Ray(new Point(light.x, light.y), Math.atan2(light.y - point.y, light.x - point.x) + Math.PI - .01, point));
                    // rays.push(new Ray(new Point(light.x, light.y), Math.atan2(light.y - point.y, light.x - point.x) + Math.PI,       point));
                    // rays.push(new Ray(new Point(light.x, light.y), Math.atan2(light.y - point.y, light.x - point.x) + Math.PI + .01, point));

                    // TODO Offset the rays by angle rather than target position
                    // Right now there is some strangeness with light piercing shapes that likely is
                    // due to this change of behavior. The larger the margins here are the less
                    // likely a float round error causes the light to pierce the shapes and mess up
                    // the polygons but the less accurately the light wraps around corners.
                    rays.push(Segment::ray(light.pos, *point));
                    rays.push(Segment::ray(light.pos, Point { x: point.x + 10.0, y: point.y }));
                    rays.push(Segment::ray(light.pos, Point { x: point.x, y: point.y + 10.0 }));
                }
            }

            // rays.sort(function (a, b) {
            //     return -a.dir + b.dir;
            // });
            rays.sort_by(|a, b| b.dir.partial_cmp(&a.dir).unwrap());

            let mut intersects: Vec<Point> = Vec::new();
            for (rayIndex, ray) in rays.iter().enumerate() {
                let mut maybe_lowest: Option<Point> = None;
                for shape in self.shapes.iter() {
                    for segment in shape.get_segments().iter() {
                        let maybe_intersect = get_line_intersect(ray, segment);

                        //  if (intersect != null && (/*shapeIndex == 0 ||*/
                        //      intersect.distanceTo(segment.start) > 0.5 &&
                        //      intersect.distanceTo(segment.end) > 0.5)) {
                        if let Some(intersect) = maybe_intersect {
                            if intersect.distance_to(&segment.start) > 0.5 && intersect.distance_to(&segment.end) > 0.5 {
                                // if (lowest == null) {
                                //     lowest = intersect;
                                // } else if (intersect.distanceTo(light) < lowest.distanceTo(light)) {
                                //     lowest = intersect;
                                // }
                                if let Some(lowest) = maybe_lowest {
                                    if intersect.distance_to(&light.pos) < lowest.distance_to(&light.pos) {
                                        maybe_lowest = Some(intersect);
                                    }
                                } else {
                                    maybe_lowest = Some(intersect);
                                }
                            }
                        }
                    }
                }

                if let Some(lowest) = maybe_lowest {
                    if rayIndex > 0 && (rayIndex - 1) % 3 == 0 && lowest.distance_to(&light.pos) > ray.end.distance_to(&light.pos) {
                        intersects.push(ray.end);
                    } else {
                        intersects.push(lowest);
                    }
                } else {
                    // let unknown = shapes[0].get_segments().find(|a| a == ray.dest)
                    // if let Some(thing) {
                    //     intersects.push(ray.dest);
                    // }
                }
            }

            light.visible = intersects;
        }
    }

    pub fn update_mouse(&mut self, position: &[f64; 2]) {
        self.lights[0].move_to(position[0], position[1]);
    }
}