use std::f64::consts::E;

use plotly::common::{Mode, Title};
use plotly::layout::{Axis, AxisType, Layout};
use plotly::{Plot, Scatter};

fn calculate_error(
    function: fn(f64) -> f64,
    analytical_derivative_function: fn(f64) -> f64, 
    point: f64, 
    h_values: &Vec<f64>,
    title: &str
) {
    let fx = function(point);
    let an_dfx = analytical_derivative_function(point);
    let num_dfxs = h_values
                    .iter()
                    .map(|h| numerical_derivative(function, point, *h))
                    .collect::<Vec<f64>>();
    let abs_errs = num_dfxs
                    .iter()
                    .map(|num_dfx| absolute_error(an_dfx, *num_dfx))
                    .collect::<Vec<f64>>();

    println!("Calculating error for function: {}", title);
    println!("f({}) = {}",point, fx);
    println!("analytical f'({}) = {}", point, an_dfx);
    println!("num f'({}) = {:?}", point, num_dfxs);

    let trace = Scatter::new(h_values.clone(), abs_errs.clone()).mode(Mode::LinesMarkers);
    let layout = Layout::new()
                          .x_axis(
                              Axis::new()
                                  .type_(AxisType::Log)
                                  .title(Title::from("h Values"))
                          )
                          .y_axis(
                              Axis::new()
                                  .type_(AxisType::Log)
                                  .title(Title::from("Absolute Error"))
                          )
                          .title(Title::from(title));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    //plot.show();
    plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);

}

fn polinomial(x: f64) -> f64 {
    -0.1 * x.powi(4) -0.15 * x.powi(3) - 0.5 * x.powi(2) - 0.25 * x + 1.2
}

fn analytical_derivative_polynomial(x: f64) -> f64 {
    -0.4 * x.powi(3) - 0.45 * x.powi(2) - x - 0.25
}

fn numerical_derivative(function: fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (function(x + h) - function(x)) / h
}

fn absolute_error(analytical: f64, numerical: f64) -> f64 {
    (analytical - numerical).abs()
}

fn exp_sinusoidal(x: f64) -> f64 {
    E.powf((2.0*x).sin())
}

fn analytical_derivative_exp_sinusoidal(x: f64) -> f64 {
    2.0 * (2.0 * x).cos() * E.powf((2.0 * x).sin())
}

fn main() {
    let x = 0.5;
    let h_values = (1..=11).map(|x| 10.0_f64.powi(-x)).collect::<Vec<f64>>();
    calculate_error(polinomial, analytical_derivative_polynomial, x, &h_values, "f(x) = -0.1x^4 - 0.15x^3 - 0.5x^2 - 0.25x + 1.2");
    calculate_error(exp_sinusoidal, analytical_derivative_exp_sinusoidal, x, &h_values, "f(x) = e^(sin(2x))");
}
