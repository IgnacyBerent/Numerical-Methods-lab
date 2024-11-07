use plotly::common::Mode;
use plotly::layout::Layout;
use plotly::{Plot, Scatter};
use roots::{find_root_brent, SimpleConvergency};

fn graphical_method(f: fn(f64)->f64) {
    let x_values = 0..=10;
    let y_values = x_values.clone().map(|x| f(x as f64));
    let trace = Scatter::new(x_values.clone().collect(), y_values.collect()).mode(Mode::LinesMarkers);
    let layout = Layout::new();
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
}

fn bisect_method(f: fn(f64) -> f64, xl: f64, xu: f64, xr_old_option: Option<f64>) {
    let xr_old = match xr_old_option {
        Some(value) => value,
        None => (xl + xu) / 2.0 + 0.1
    };

    if f(xl) * f(xu) > 0.0 {
        println!("The function has the same sign at the end points");
        return;
    } else if f(xl) * f(xu) < 0.0 {
        let xr = (xl + xu) / 2.0;
        let error = (xr - xr_old).abs() / xr;
        if -error < f(xl)*f(xr) && f(xr)*f(xr) < error {
            println!("Result for Bisection is: {}", xr);
            println!("Error: {:e}", f(xr));
            return;
        } else if f(xl) * f(xr) < 0.0 {
            return bisect_method(f, xl, xr, Some(xr));
        } else if f(xr) * f(xu) < 0.0 {
            return bisect_method(f, xr, xu, Some(xr));
        }
    } else if f(xl) == 0.0 {
        println!("The root is at xl");
        return;
    } else if f(xu) == 0.0 {
        println!("The root is at xu");
        return;
    }
}


fn newton_method(f: fn(f64) -> f64, x: f64, tol: f64, max_iter: usize) {
    let mut x0 = x;
    for _ in 0..max_iter {
        let dfx = derivative(f, x0);
        if dfx == 0.0 {
            println!("The derivative is zero");
            return;
        }
        let x1 = x0 - f(x0) / dfx;
        if (x1 - x0).abs() < tol*x1.abs() {
            println!("Result for Newotn Mehod is: {}", x1);
            println!("Error: {:e}", f(x1));
            return;
        }
        x0 = x1;
    }
    println!("The method did not converge");
}


fn derivative(f: fn(f64) -> f64, x: f64) -> f64 {
    let h = 10e-7;
    (f(x + h/2.0) - f(x - h/2.0)) / h
}

fn library_method(a:f64, b:f64, f: fn(f64) -> f64) {
    let result = find_root_brent(a, b, &f, &mut SimpleConvergency { eps: 1e-7, max_iter: 100 });
    match result {
        Ok(root) => println!("Result for library method is: {} \n Error {:e}", root, f(root)),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn example_funciton(h: f64) -> f64 {
    let g = 9.81;
    let l = 5.0;
    let t = 3.0;
    let vt = 4.0;
    let sqrt_op = (2.0 * g * h).sqrt();
    let v_h = sqrt_op * tanh(sqrt_op / (2.0*l) * t) - vt; 
    return v_h;
}

fn tanh(x: f64) -> f64 {
    (x.exp() - (-x).exp()) / (x.exp() + (-x).exp())
}

fn main() {
    let function = example_funciton;
    graphical_method(function);
    bisect_method(function, 0.0, 10.0, None);
    newton_method(function, 2.0, 0.001, 100);
    library_method(0.0, 10.0, function);
}
