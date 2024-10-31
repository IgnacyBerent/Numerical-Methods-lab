use plotly::common::{Mode, Title};
use plotly::layout::{Axis, AxisType, Layout};
use plotly::{Plot, Scatter};

fn plot_error_vs_h(
    h_values: Vec<f64>, 
    abs_errs_central: Vec<f64>,
    abs_errs_forward: Vec<f64>, 
    opt_h_cent: f64, 
    opt_err_cent: f64,
    title: &str
    ) {
    let trace_central = Scatter::new(h_values.clone(), abs_errs_central.clone()).mode(Mode::LinesMarkers).name("Central");
    let trace_forward = Scatter::new(h_values.clone(), abs_errs_forward.clone()).mode(Mode::LinesMarkers).name("Forward");
    let trace_optimal = Scatter::new(vec![opt_h_cent], vec![opt_err_cent]).mode(Mode::Markers).name("Optimal");
    let layout = Layout::new()
        .x_axis(Axis::new().type_(AxisType::Log).title(Title::from("h Values")))
        .y_axis(Axis::new().type_(AxisType::Log).title(Title::from("Absolute Error")))
        .title(Title::from(title));
    let mut plot_cfg = Plot::new();
    plot_cfg.add_trace(trace_central);
    plot_cfg.add_trace(trace_forward);
    plot_cfg.add_trace(trace_optimal);
    plot_cfg.set_layout(layout);
    plot_cfg.show_image(plotly::ImageFormat::JPEG, 1000, 800);
}

fn plot_error_vs_h_second_der(
    h_values: Vec<f64>,
    abs_err: Vec<f64>,
    title: &str
) {
    let trace = Scatter::new(h_values.clone(), abs_err.clone()).mode(Mode::LinesMarkers);
    let layout = Layout::new()
        .x_axis(Axis::new().type_(AxisType::Log).title(Title::from("h Values")))
        .y_axis(Axis::new().type_(AxisType::Log).title(Title::from("Absolute Error")))
        .title(Title::from(title));
    let mut plot_cfg = Plot::new();
    plot_cfg.add_trace(trace);
    plot_cfg.set_layout(layout);
    plot_cfg.show_image(plotly::ImageFormat::JPEG, 1000, 800);
}

fn two_point_central_der(f: fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (f(x + h/2.0) - f(x - h/2.0)) / h
}

fn two_point_foward_der(function: fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (function(x + h) - function(x)) / h
}

fn second_derivative(f: fn(f64) -> f64, x: f64, h: f64) -> f64 {
    4.0*(f(x + h/2.0)+f(x-h/2.0)-2.0*f(x))/h.powi(2) 
}

fn absolute_error(analytical: f64, numerical: f64) -> f64 {
    (analytical - numerical).abs()
}

fn h_optimal_central() -> f64 {
    std::f64::EPSILON.powf(1.0 / 3.0)
}

fn function_1(x: f64) -> f64 {
    std::f64::consts::E.powf((2.0*x).sin())
}

fn analytical_derivative_1(x: f64) -> f64 {
    2.0 * (2.0*x).cos() * std::f64::consts::E.powf((2.0*x).sin())
}

fn analytical_second_derivative_1(x: f64) -> f64 {
    let sin_2x = (2.0*x).sin();
    let cos_2x = (2.0*x).cos();
    4.0 * (cos_2x.powi(2) - sin_2x) * std::f64::consts::E.powf(sin_2x)
}

fn function_2(x: f64) -> f64 {
    x.powi(3) - 2.0*x.powi(2) + 3.0*x - 1.0
}

fn analytical_derivative_2(x: f64) -> f64 {
    3.0*x.powi(2) - 4.0*x + 3.0
}

fn task_1(x: f64, h_values: Vec<f64>){
    let analytical = analytical_derivative_1(x);
    let num_dfxs_central = h_values.iter().map(|h| two_point_central_der(function_1, x, *h)).collect::<Vec<f64>>();
    let num_dfxs_forward = h_values.iter().map(|h| two_point_foward_der(function_1, x, *h)).collect::<Vec<f64>>();
    let abs_errs_central = num_dfxs_central.iter().map(|num_dfx| absolute_error(analytical, *num_dfx)).collect::<Vec<f64>>();
    let abs_errs_forward = num_dfxs_forward.iter().map(|num_dfx| absolute_error(analytical, *num_dfx)).collect::<Vec<f64>>();
    let h_optimal = h_optimal_central();
    let num_dfx_optimal = two_point_central_der(function_1, x, h_optimal);
    let abs_err_optimal = absolute_error(analytical, num_dfx_optimal);
    println!("Optimal h for central difference: {}", h_optimal);
    println!("Optimal numerical derivative: {}", num_dfx_optimal);
    println!("Optimal absolute error: {}", abs_err_optimal);
    plot_error_vs_h(h_values, abs_errs_central, abs_errs_forward, h_optimal, abs_err_optimal, "First derrivative of e^(sin(2x))");
}

fn task_2(x: f64, h_values: Vec<f64>){
    let analytical = analytical_second_derivative_1(x);
    let num_dfxs = h_values.iter().map(|h| second_derivative(function_1, x, *h)).collect::<Vec<f64>>();
    let abs_errs = num_dfxs.iter().map(|num_dfx| absolute_error(analytical, *num_dfx)).collect::<Vec<f64>>();
    plot_error_vs_h_second_der(h_values, abs_errs, "Second derrivative of e^(sin(2x))");
}

fn task_3(x: f64, h_values: Vec<f64>){
    let analytical = analytical_derivative_2(x);
    let num_dfxs_central = h_values.iter().map(|h| two_point_central_der(function_2, x, *h)).collect::<Vec<f64>>();
    let num_dfxs_forward = h_values.iter().map(|h| two_point_foward_der(function_2, x, *h)).collect::<Vec<f64>>();
    let abs_errs_central = num_dfxs_central.iter().map(|num_dfx| absolute_error(analytical, *num_dfx)).collect::<Vec<f64>>();
    let abs_errs_forward = num_dfxs_forward.iter().map(|num_dfx| absolute_error(analytical, *num_dfx)).collect::<Vec<f64>>();
    let h_optimal = h_optimal_central();
    let num_dfx_optimal = two_point_central_der(function_2, x, h_optimal);
    let abs_err_optimal = absolute_error(analytical, num_dfx_optimal);
    println!("Optimal h for central difference: {}", h_optimal);
    println!("Optimal numerical derivative: {}", num_dfx_optimal);
    println!("Optimal absolute error: {}", abs_err_optimal);
    plot_error_vs_h(h_values, abs_errs_central, abs_errs_forward, h_optimal, abs_err_optimal, "First derrivative of x^3 - 2x^2 + 3x - 1");
}

fn main() {
    let x = 0.5;
    let h_values = (1..=11).map(|n| 10.0_f64.powi(-n)).collect::<Vec<f64>>();
    task_1(x, h_values.clone());
    task_2(x, h_values.clone());
    task_3(x, h_values.clone());
}
