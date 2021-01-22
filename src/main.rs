use nannou::{color::named, prelude::*};
use crate::Color::DarkGoldenrod;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

trait Etch {
    fn display(&self, draw: &Draw, time: f32);
    fn update(&mut self);
}

#[derive(Debug,Clone,Copy)]
enum Color {
    DarkGoldenrod,
    SeaGreen,
}
impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}
impl From<Color> for Srgb<u8> {
    fn from(c: Color) -> Self {
        named::from_str(&c.to_string()).unwrap()
    }
}

type Point = Point2;
struct Splotch {
    locus: Point,
    width: f32,
    length: f32,
    color: Color,
    turn: f32,
    swerve: f32,
    win_rect: Rect,
}
impl Splotch {
    fn new(win_rect: Rect) -> Self {
        Self{
            locus: Point::default(),
            width: 25.0,
            length: 50.0,
            color: Color::DarkGoldenrod,
            turn: 0.0,
            swerve: 0.02,
            win_rect,
        }
    }
    fn scale(&self) {

    }
}
impl Etch for Splotch {
    fn display(&self, draw: &Draw, time: f32) {
        let f = (1.0-time/10.0).max(0.0);
        let d = draw.scale(f);
        d.ellipse()
            .w(self.width)
            .h(self.length)
            .xy(self.locus)
            .z_turns(self.turn)
            .color(self.color(f));
        let step = Vector2::new(0.0, 20.0).rotate(self.turn*6.2832);
        d.ellipse()
            .w(self.width/2.0)
            .h(self.length/2.0)
            .xy(self.locus+step)
            .z_turns(self.turn)
            .color(self.color(f));
    }

    fn update(&mut self) {
        self.turn += random_range(-self.swerve, self.swerve);
        let step = Vector2::new(0.0, 20.0).rotate(self.turn*6.2832);
        self.locus += step;
        if self.locus.x < self.win_rect.left() {
            // println!("Too left: {} {}", self.locus.x, self.win_rect.left());
            self.locus.x += self.win_rect.w()
        } else if self.locus.x > self.win_rect.right() {
            // println!("Too right: {} {}", self.locus.x, self.win_rect.right());
            self.locus.x -= self.win_rect.w()
        }
        if self.locus.y < self.win_rect.bottom() {
            // println!("Too down: {} {}", self.locus.y, self.win_rect.bottom());
            self.locus.y += self.win_rect.h()
        } else if self.locus.y > self.win_rect.top() {
            self.locus.y -= self.win_rect.h()
        }
    }
}
impl Splotch {
    fn color(&self, factor: f32) -> Srgba<f32> {
        Srgba::new((3.0 * self.turn).sin(), (7.0 * self.turn).sin(), (11.0 * self.turn).sin(), 0.32*factor)
    }
}

struct Model {
    // nsplotches: u32,
    splotches: Vec<Splotch>,
}

fn model(app: &App) -> Model {
    app.draw().background().color(Rgb::from(Color::SeaGreen));
    Model::new(app)
}

impl Etch for Model {
    fn display(&self, draw: &Draw<f32>, time: f32) {
        self.splotches.iter().for_each(|s|s.display(draw, time))
    }

    fn update(&mut self) {
        self.splotches.iter_mut().for_each(|s|s.update())
    }
}
impl Model {
    fn new(app: &App) -> Self {
        Self {
            splotches: (0..127).map(|_|Splotch::new(app.window_rect())).collect()
        }
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let canvas = app.draw();
    model.display(&canvas, app.time);
    canvas.to_frame(app, &frame).unwrap();
}
