use nannou::prelude::*;

fn main() {
    nannou::app(model).simple_window(view).run();
}
struct Wander {
    ux: f32,
    uy: f32,
    w: f32,
    h: f32,
}
impl Wander {
    fn display(canvas: &Draw) {

    }
}

struct Model {
}
fn model(app: &App) -> Model {
    Model{}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // get canvas to draw on
    let sint = app.time.sin();
    let t = app.time;

    let canvas = app.draw();
    if frame.nth() == 0 {
        canvas.background().color(PURPLE);
    }
    let win = app.window_rect();
    let w = win.w();
    let h = win.h();

    let rw = (100.0-t*5.0);
    let rh = (100.0-t*5.0);
    if rw < 1.0 {
        return
    }
    let rec = Rect::from_w_h(rw, rh);
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut r: u8 = 0;
    let mut g: u8 = 0;
    let mut b: u8 = 0;
    let mut a: u8 = 0;

    for i in 0..=128 {
        let fi = (i as f32)/128.0;
        x = (x+37.7*fi+w/2.0+ t*t)%w - w/2.0 + (10*(i%10-5)) as f32;
        y = (y+67.7*fi+h/2.0 + t*t)%h - h/2.0;
        r = r+199*((255.0*x) as u8);
        g = g+37;
        b = b+100;
        a = 25;
        canvas.ellipse()
            .xy(rec.xy())
            .w_h(rec.w()/2.0, rec.h())
            .x_y(x, y)
            .z_degrees(360.0*(sint+x/w))
            .color(rgba8(255-r, 255-g, 255-b, a));

        canvas.rect()
            .xy(rec.xy())
            .wh(rec.wh())
            .x_y(x, y)
            .z_degrees(360.0 * (sint+x/w))
            .color(rgba8(r, g, b, a));
    }

    // put everything on the frame
    canvas.to_frame(app, &frame).unwrap();
}
