use plotly::common::Title;
use plotly::layout::{Axis, AxisType, Layout};
use plotly::{Plot, Scatter};

fn f1(x: f64, v: f64) -> f64 {
    return -x;
}

fn f2(y: f64, v: f64) -> f64 {
    return v;
}

fn euler_method(x0: f64, v0: f64, dt: f64) -> (f64, f64) {
    let x1 = x0 + f2(x0, v0) * dt;
    let v1 = v0 + f1(x0, v0) * dt;
    return (x1, v1);
}
fn main() {
    let (x0, v0) = (1.0, 0.0);
    let dt = 0.001;
    let n = 10.0;
    let mut t = 0.0;
    let mut x_axis = vec![1.0];
    let mut y_axis = vec![0.0];
    let mut x = x0;
    let mut y = v0;

    while t < n {
        let (x1, v1) = euler_method(x, y, dt);
        x = x1;
        y = v1;
        x_axis.push(x1);
        y_axis.push(v1);
        t += dt;
    }

    let trace = Scatter::new(x_axis, y_axis).name("Euler method");
    let layout = Layout::new()
        .title(Title::from("Euler method"))
        .x_axis(Axis::new().title(Title::from("x")))
        .y_axis(Axis::new().title(Title::from("v")));
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
}
