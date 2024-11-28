use plotly::common::Title;
use plotly::layout::{Axis, AxisType, Layout};
use plotly::{Plot, Scatter};
use std::f64::consts::E;

const TEORETICAL_VALUE: f64 = 75.33896260915866;
const X: f64 = 4.0;
const Y0: f64 = 2.0;

fn equation(x: f64, y: f64) -> f64 {
    return 4.0 * E.powf(0.8 * x) - 0.5 * y;
}

fn euler_method(x0: f64, h: f64, n: i32) -> f64 {
    let mut x = x0;
    let mut y = Y0;
    for _i in 0..n {
        y = y + h * equation(x, y);
        x = x + h;
    }
    return y;
}

fn modified_euler_method(x0: f64, h: f64, n: i32) -> f64 {
    let mut x = x0;
    let mut y = Y0;
    for _i in 0..n {
        let k1 = equation(x, y);
        let k2 = equation(x + h, y + h * k1);
        y = y + h * (k1 + k2) / 2.0;
        x = x + h;
    }
    return y;
}

fn runge_kutta_method(x0: f64, h: f64, n: i32) -> f64 {
    let mut x = x0;
    let mut y = Y0;
    for _i in 0..n {
        let k1 = equation(x, y);
        let k2 = equation(x + h / 2.0, y + h * k1 / 2.0);
        let k3 = equation(x + h / 2.0, y + h * k2 / 2.0);
        let k4 = equation(x + h, y + h * k3);
        y = y + h * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
        x = x + h;
    }
    return y;
}

fn relative_error(result: f64) -> f64 {
    return ((TEORETICAL_VALUE - result) / TEORETICAL_VALUE).abs() * 100.0;
}

fn plot_logarithm_of_error(
    n_vec: Vec<i32>,
    solve_method: fn(f64, f64, i32) -> f64,
    computional_cost: u8,
    trace_name: &str,
) -> Box<Scatter<i32, f64>> {
    let computional_cost_vec = n_vec
        .iter()
        .map(|n| computional_cost as i32 * n)
        .collect::<Vec<i32>>();
    let y_errors = n_vec
        .iter()
        .map(|n| {
            let h = X / *n as f64;
            let y = solve_method(0.0, h, *n);
            relative_error(y)
        })
        .collect::<Vec<f64>>();

    let trace: Box<Scatter<i32, f64>> =
        Scatter::new(computional_cost_vec, y_errors).name(trace_name);
    return trace;
}

fn plot_together(traces: Vec<Box<Scatter<i32, f64>>>) {
    let mut plot = Plot::new();
    let layout = Layout::new()
        .x_axis(
            Axis::new()
                .type_(AxisType::Log)
                .title(Title::from("Computional cost")),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Log)
                .title(Title::from("Relative error")),
        )
        .title(Title::from("Logarithm of relative error for all methods"));
    for trace in traces {
        plot.add_trace(trace);
    }
    plot.set_layout(layout);
    plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
}

fn main() {
    let n_vec = (0..11).map(|x| 2_i32.pow(x)).collect::<Vec<i32>>();
    let eul = plot_logarithm_of_error(n_vec.clone(), euler_method, 1, "Euler");
    let meul = plot_logarithm_of_error(n_vec.clone(), modified_euler_method, 2, "Modified Euler");
    let rk = plot_logarithm_of_error(n_vec.clone(), runge_kutta_method, 4, "Runge-Kutta");
    plot_together(vec![eul, meul, rk]);
}
