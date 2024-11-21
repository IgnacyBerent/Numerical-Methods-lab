use plotly::common::{Mode, Title};
use plotly::layout::{Axis, AxisType, Layout};
use plotly::{Plot, Scatter};

fn task() {
    let x_range = (0.0, 1.0);
    let n_vec = (1..=7).map(|n| 10_i32.pow(n)).collect::<Vec<i32>>();
    let f = function_1;
    let integral_analitical = function_1_analitical_integral(x_range);
    plot_integration(
        n_vec.clone(),
        integral_linear,
        "Linear",
        integral_analitical,
        f,
        x_range,
    );
    plot_integration(
        n_vec.clone(),
        integral_trapezoidal,
        "Trapezoidal",
        integral_analitical,
        f,
        x_range,
    );
    plot_integration(
        n_vec.clone(),
        simpson13,
        "Simpson 1/3",
        integral_analitical,
        f,
        x_range,
    );
}

fn plot_integration(
    n_vec: Vec<i32>,
    method: fn((f64, f64), f64, fn(f64) -> f64) -> f64,
    method_name: &str,
    analitycal_result: f64,
    f: fn(f64) -> f64,
    x_range: (f64, f64),
) {
    let mut x = Vec::new();
    let mut y_error = Vec::new();
    for n in n_vec {
        let integral = method(x_range, n as f64, f);
        let error = absolute_error(integral, analitycal_result);
        x.push(n);
        y_error.push(error);
    }
    let trace = Scatter::new(x, y_error)
        .mode(Mode::LinesMarkers)
        .name(format!("{} error", method_name));
    let layout = Layout::new()
        .x_axis(Axis::new().type_(AxisType::Log).title(Title::from("n")))
        .y_axis(
            Axis::new()
                .type_(AxisType::Log)
                .title(Title::from("Absolute Error")),
        );
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
}

fn absolute_error(integral: f64, integral_analitical: f64) -> f64 {
    (integral - integral_analitical).abs()
}

fn function_1(x: f64) -> f64 {
    let fx = 1.0 / ((x.powi(2) + 1.0).sqrt());
    fx
}

fn function_1_analitical_integral(x_range: (f64, f64)) -> f64 {
    x_range.1.asinh() - x_range.0.asinh()
}

fn integral_linear(x_range: (f64, f64), n: f64, f: fn(f64) -> f64) -> f64 {
    let h = (x_range.1 - x_range.0) / n;
    let mut sum = 0.0;
    let x_start = x_range.0;
    let x_end = x_range.1;
    let mut x_i = x_start;
    while x_i < x_end {
        sum += f(x_i) * h;
        x_i += h;
    }
    sum
}

fn integral_trapezoidal(x_range: (f64, f64), n: f64, f: fn(f64) -> f64) -> f64 {
    let h = (x_range.1 - x_range.0) / n;
    let mut sum = 0.0;
    let x_start = x_range.0;
    let x_end = x_range.1;
    let mut x_i = x_start;
    while x_i < x_end {
        sum += (f(x_i) + f(x_i + h)) * h / 2.0;
        x_i += h;
    }
    sum
}

fn simpson13(x_range: (f64, f64), n: f64, f: fn(f64) -> f64) -> f64 {
    let h = (x_range.1 - x_range.0) / n;
    let mut sum = 0.0;
    let x_start = x_range.0;
    let x_end = x_range.1;
    let mut x_i = x_start;
    while x_i < x_end {
        sum += (f(x_i) + 4.0 * f(x_i + h / 2.0) + f(x_i + h)) * h / 6.0;
        x_i += h;
    }
    sum
}

fn main() {
    task();
}
