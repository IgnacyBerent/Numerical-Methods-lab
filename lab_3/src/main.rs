use libm::nextafter;
use plotly::common::{Mode, Title};
use plotly::layout::{Axis, AxisType, Layout};
use plotly::{Plot, Scatter};

fn task_1() {
    let mut precision: f64 = 1.0;
    let mut anwser: f64 = 0.0;
    while 1.0 - precision != 1.0 {
      anwser = precision;
      precision /= 2.0;
    }
    println!("Precision: {:e}", anwser);
}

fn task_2() {
  let x_start = -300;
  let x_end = 300;
  let n = 100;
  let (x_values, deltax_values) = generate_data(x_start, x_end, n);

  let trace = Scatter::new(x_values, deltax_values).mode(Mode::LinesMarkers);
  let layout = Layout::new()
                        .x_axis(
                            Axis::new()
                                .type_(AxisType::Log)
                                .title(Title::from("X Values"))
                        )
                        .y_axis(
                            Axis::new()
                                .type_(AxisType::Log)
                                .title(Title::from("Delta x"))
                        )
                        .title(Title::from(""));
  let mut plot = Plot::new();
  plot.add_trace(trace);
  plot.set_layout(layout);
  plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
}

fn generate_data(start: i32, end: i32, n: i32) -> (Vec<f64>, Vec<f64>) {
    let values = logspace(start, end, n);
    let delta_values = values
                                  .iter()
                                  .map(|x| nextafter(*x, f64::INFINITY) - x)
                                  .collect::<Vec<f64>>();
    (values, delta_values)
}

fn logspace(start: i32, end: i32, n: i32) -> Vec<f64> {
    let base: f64 = 10.0;
    let mut result: Vec<f64> = Vec::new();
    let step = (end - start) / n;
    for i in 0..=n {
        result.push(base.powi(start + i * step));
    }
    result
}

fn task_3() {
  let x_start = -300;
  let x_end = 300;
  let n = 100;
  let (x_values, deltax_values) = generate_data(x_start, x_end, n);
  let rel_err_x = x_values.iter().zip(deltax_values.iter()).map(|(x, dx)| dx / x).collect::<Vec<f64>>();

  let trace = Scatter::new(x_values, rel_err_x).mode(Mode::LinesMarkers);
  let layout = Layout::new()
                        .x_axis(
                            Axis::new()
                                .type_(AxisType::Log)
                                .title(Title::from("X Values"))
                        )
                        .y_axis(
                            Axis::new()
                                .type_(AxisType::Log)
                                .title(Title::from("Relative Error"))
                        )
                        .title(Title::from(""));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
}


fn main() {
    task_1();
    task_2();
    task_3();
}
