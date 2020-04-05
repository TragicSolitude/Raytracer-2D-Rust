use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use std::f64::consts::PI;
use graphics::math::Vec2d;
use std::ops::Deref;
use graphics::types::Rectangle;
use graphics::ellipse::circle;

trait Positioned {
    fn x(&self) -> f64;
    fn y(&self) -> f64;

    fn distance_to(&self, other: &impl Positioned) -> f64 {
        ((other.x() - self.x()).powi(2) + (other.y() - self.y()).powi(2)).sqrt()
    }
}

impl Positioned for Vec2d {
    fn x(&self) -> f64 {
        self[0]
    }

    fn y(&self) -> f64 {
        self[1]
    }
}

#[derive(Clone)]
struct LightSource {
    pub pos: Vec2d,
    pub visible: Vec<Vec2d>,
    // color omitted
    // all omitted
}

impl LightSource {
    pub fn new() -> Self {
        LightSource {
            pos: [0.0, 0.0],
            visible: Vec::new()
        }
    }

    /// Mutable builder style
    pub fn move_to(&mut self, new_position: Vec2d) -> &mut Self {
        self.pos = new_position;

        self
    }

    /// One-liner builder style
    pub fn at_position(mut self, new_position: Vec2d) -> Self {
        self.pos = new_position;

        self
    }
}

impl Deref for LightSource {
    type Target = Vec2d;

    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

struct Segment {
    pub start: Vec2d,
    pub end: Vec2d,
    pub dir: f64
}

impl Segment {
    pub fn new(p1: Vec2d, p2: Vec2d) -> Self {
        // Direction memoized
        let dir = (p2.y() - p1.y()).atan2(p2.x() - p1.x());

        Segment { start: p1, end: p2, dir }
    }

    // moved Ray subclass to secondary constructor on Segment
    // omitted dest argument
    pub fn ray(p1: Vec2d, dir: f64) -> Self {
        // super(p1, new Point(p1.x + Math.cos(dir) * 2147483647, p1.y + Math.sin(dir) * 2147483647), dir);
        let end = [
            p1.x() + dir.cos() * 2147483647.0,
            p1.y() + dir.sin() * 2147483647.0
        ];

        Segment { start: p1, end, dir }
    }
}

trait Rectangular {
    fn vertices(&self) -> [Vec2d; 4];

    fn edges(&self) -> [Segment; 4] {
        let points = self.vertices();

        [
            Segment::new(points[0], points[1]),
            Segment::new(points[1], points[2]),
            Segment::new(points[2], points[3]),
            Segment::new(points[3], points[0])
        ]
    }
}

impl Rectangular for graphics::types::Rectangle {
    fn vertices(&self) -> [Vec2d; 4] {
        [
            [self[0], self[1]],
            [self[0] + self[2], self[1]],
            [self[0] + self[2], self[1] + self[3]],
            [self[0], self[1] + self[3]]
        ]
    }
}

// TODO Rewrite this monstrosity as a trait fn
fn calculate_line_intersect(p0_x: f64, p0_y: f64, p1_x: f64, p1_y: f64, p2_x: f64, p2_y: f64, p3_x: f64, p3_y: f64) -> Option<Vec2d> {
    let s1_x = p1_x - p0_x;
    let s1_y = p1_y - p0_y;
    let s2_x = p3_x - p2_x;
    let s2_y = p3_y - p2_y;

    let s = (-s1_y * (p0_x - p2_x) + s1_x * (p0_y - p2_y)) / (-s2_x * s1_y + s1_x * s2_y);
    let t = ( s2_x * (p0_y - p2_y) - s2_y * (p0_x - p2_x)) / (-s2_x * s1_y + s1_x * s2_y);

    if s>= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
        let i_x = p0_x + (t * s1_x);
        let i_y = p0_y + (t * s1_y);

        return Some([i_x, i_y]);
    }

    None
}

fn get_line_intersect(s1: &Segment, s2: &Segment) -> Option<Vec2d> {
    calculate_line_intersect(s1.start.x(), s1.start.y(), s1.end.x(), s1.end.y(), s2.start.x(), s2.start.y(), s2.end.x(), s2.end.y())
}

pub struct App {
    gl: GlGraphics,
    lights: Vec<LightSource>,
    shapes: Vec<Rectangle>
}

impl App {
    pub fn new(gl: GlGraphics) -> Self {
        let lights = vec![
            LightSource::new().at_position([485.0, 485.0])
        ];

        let shapes = vec![
            [0.0, 0.0, 750.0, 750.0],
            [30.0, 30.0, 80.0, 80.0],
            [400.0, 80.0, 60.0, 120.0],
            [300.0, 550.0, 350.0, 50.0]
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
        const COLOR_LIGHTSOURCE: [f32; 4] = [255.0, 255.0, 255.0, 0.2];

        let shapes = self.shapes.clone();
        let lights = self.lights.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(COLOR_BACKGROUND, gl);

            for shape in shapes.iter().skip(1) {
                rectangle(COLOR_SHAPES, *shape, c.transform, gl);
            }

            for light in lights.iter() {
                polygon(COLOR_LIGHT, &light.visible, c.transform, gl);

                let lightsource_circle = circle(light.pos.x(), light.pos.y(), 20.0);
                ellipse(COLOR_LIGHTSOURCE, lightsource_circle, c.transform, gl);
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        for light in self.lights.iter_mut() {
            let mut rays = Vec::with_capacity(self.shapes.len() * 4 * 3);

            for shape in self.shapes.iter() {
                for vertex in shape.vertices().iter() {
                    // rays.push(new Ray(new Point(light.x, light.y), Math.atan2(light.y - point.y, light.x - point.x) + Math.PI - .01, point));
                    // rays.push(new Ray(new Point(light.x, light.y), Math.atan2(light.y - point.y, light.x - point.x) + Math.PI,       point));
                    // rays.push(new Ray(new Point(light.x, light.y), Math.atan2(light.y - point.y, light.x - point.x) + Math.PI + .01, point));

                    let dir = (light.pos.y() - vertex.y()).atan2(light.pos.x() - vertex.x()) + PI;
                    rays.push(Segment::ray(light.pos, dir - 0.01));
                    rays.push(Segment::ray(light.pos, dir));
                    rays.push(Segment::ray(light.pos, dir + 0.01));
                }
            }

            // rays.sort(function (a, b) {
            //     return -a.dir + b.dir;
            // });
            rays.sort_by(|a, b| b.dir.partial_cmp(&a.dir).unwrap());

            let mut intersects: Vec<Vec2d> = Vec::with_capacity(rays.len() * 2);
            intersects.push(light.pos);
            for (ray_index, ray) in rays.iter().enumerate() {
                let mut maybe_lowest: Option<Vec2d> = None;
                for shape in self.shapes.iter() {
                    for edge in shape.edges().iter() {
                        let maybe_intersect = get_line_intersect(ray, edge);

                        //  if (intersect != null && (/*shapeIndex == 0 ||*/
                        //      intersect.distanceTo(segment.start) > 0.5 &&
                        //      intersect.distanceTo(segment.end) > 0.5)) {
                        if let Some(intersect) = maybe_intersect {
                            if intersect.distance_to(&edge.start) > 0.5 && intersect.distance_to(&edge.end) > 0.5 {
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
                    if ray_index > 0 && (ray_index - 1) % 3 == 0 && lowest.distance_to(&light.pos) > ray.end.distance_to(&light.pos) {
                        intersects.push(ray.end);
                    } else {
                        intersects.push(lowest);
                    }
                }
            }

            // Loop around intersects array to properly close off polygon
            intersects.push(intersects[1]);

            light.visible = intersects;
        }
    }

    pub fn update_mouse(&mut self, position: &[f64; 2]) {
        self.lights[0].move_to(*position);
    }
}