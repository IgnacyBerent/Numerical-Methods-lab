use plotly::common::{Mode, Title};
use plotly::layout::{Axis, AxisType, Layout};
use plotly::{Plot, Scatter};

fn second_derivative(f: fn(f64) -> f64, x: f64, h: f64) -> f64 {
    4.0 * (f(x + h / 2.0) + f(x - h / 2.0) - 2.0 * f(x)) / h.powi(2)
}

fn absolute_error(analytical: f64, numerical: f64) -> f64 {
    (analytical - numerical).abs()
}

fn polynomial(x: f64) -> f64 {
    x.powi(3) - 2.0 * x.powi(2) + 3.0 * x - 1.0
}

fn analytical_second_derivative_polynomial(x: f64) -> f64 {
    6.0 * x - 4.0
}

fn solution(x: f64, h_values: Vec<f64>) {
    let analytical = analytical_second_derivative_polynomial(x);
    let num_dfxs = h_values
        .iter()
        .map(|h| second_derivative(polynomial, x, *h))
        .collect::<Vec<f64>>();
    let abs_errs = num_dfxs
        .iter()
        .map(|num_dfx| absolute_error(analytical, *num_dfx))
        .collect::<Vec<f64>>();
    let mut plot = Plot::new();
    let trace = Scatter::new(h_values.clone(), abs_errs.clone()).mode(Mode::LinesMarkers);
    let layout = Layout::new()
        .x_axis(
            Axis::new()
                .type_(AxisType::Log)
                .title(Title::from("h Values")),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Log)
                .title(Title::from("Absolute Error")),
        )
        .title(Title::from("Second Derivative of Polynomial"));
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
}

fn main() {
    let x = 0.5;
    let h_values = (1..=11).map(|n| 10.0_f64.powi(-n)).collect::<Vec<f64>>();
    solution(x, h_values);
}
