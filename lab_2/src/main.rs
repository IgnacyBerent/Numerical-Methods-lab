use std::f64::consts::PI;


fn task_1() {
    let n = 94906266;
    for i in [1,2,4,8] {
        println!("Reverse sum for {}n: ", i);
        backward_sum(n*i);
    }
}

fn backward_sum(n: i32) {
    let expected = (PI*PI)/6.0;
    let mut sum = 0.0;
    for i in 0..n {
        let k = (n - i) as f64;
        sum += 1.0/(k*k) as f64;
    }
    let rel_error = (sum - expected).abs() / expected;
    println!("Sum: {}", sum);
    println!("Relative error: {}", rel_error);
}


fn main() {
    task_1();
}
