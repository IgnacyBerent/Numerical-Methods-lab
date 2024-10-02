use std::time::Instant;

fn solution() -> (f64, i32) {
    let mut sol = 0.0;
    let mut k = 1;
    let mut check = 0.0;
    loop {
        let partial_sol = 1.0 / (k as f64).powi(2);
        sol += partial_sol;
        if sol == check {
            return (sol, k);
        }
        check = sol;
        k += 1;
    }
}

fn main() {
    let start = Instant::now();
    let (sol, k) = solution();
    let duration = start.elapsed();
    println!("Rust solution:");
    println!("Solution: {}", sol);
    println!("n_max: {}", k);
    println!("Time elapsed: {:?}", duration);
    let percentage_error = (sol - std::f64::consts::PI.powi(2) / 6.0) / (std::f64::consts::PI.powi(2) / 6.0) * 100.0;
    println!("Percentage error: {}%", percentage_error);
}