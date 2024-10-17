use std::f64::consts::PI;
use std::f64::consts::E;


fn calc_rel_error(expected: f64, actual: f64) {
    let rel_error = (actual - expected).abs() / expected;
    println!("Relative error: {:e} \n", rel_error);
}

fn basel_sum_backward(n: i32) -> f64 {
    let mut sum = 0.0;
    for i in 0..n {
        let k = (n - i) as f64;
        sum += 1.0/(k*k) as f64;
    }
    sum
}

fn factorial(n: u128) -> Option<u128> {
    if n == 0 || n == 1 {
        return Some(1);
    }
    let result = n.checked_mul(factorial(n - 1)?);
    return Some(result?)
}

fn maclaurin_foward(x: f64) -> f64 {
    let mut i = 1;
    let mut sum = 1.0;
    let mut check = 1.0;
    loop {
        match factorial(i) {
            Some(fact) => {
                if x.is_sign_positive() {
                    sum += x.powi(i as i32) / fact as f64;
                }
                else {
                    sum += (-x).powi(i as i32) / fact as f64;
                }
            },
            None => break,
        }
        if sum == check {
            break;
        }
        check = sum;
        i += 1;
    }

    if x.is_sign_positive() {
        return sum;
    } else {
        return 1_f64 / sum;
    }
}


fn basel_sum_foward(n: i32) -> f64 {
    let mut sum = 0.0;
    
    for k in 1..=n {
        let number = 1.0 / (k as f64).powi(2);
        sum += number;
    }
    return sum;
}

fn basel_sum_kahal(n: i32) -> f64 {
    let mut sum = 0.0;
    let mut compensation = 0.0;

    for k in 1..=n {
        let number = 1.0 / (k as f64).powi(2);
        let y = number - compensation;
        let t = sum + y;
        compensation = (t - sum) - y;
        sum = t;
    }
    return sum;
}

fn task_1() {
    let n = 94906266;
    let expected = (PI*PI)/6.0;
    for i in [1,2,4,8] {
        println!("Reverse sum for {}n: ", i);
        let sum = basel_sum_backward(n*i);
        println!("Sum: {:e}", sum);
        calc_rel_error(expected, sum);
    }
}

fn task_2() {
    let x_to_calc = vec![0.1, 20.0, -20.0];
    for x in x_to_calc {
        println!("Maclaurin series for x = {}", x);
        println!("Expected: {:e}", E.powf(x));
        let sum = maclaurin_foward(x);
        println!("Sum: {:e}", sum);
        calc_rel_error(E.powf(x), sum);
    }
}

fn task_3() {
    let n = 94906266;
    let expected = (PI*PI)/6.0;
    for i in [1,2,4,8] {
        println!("Forward sum for {}n: ", i);
        let sum = basel_sum_foward(n*i);
        println!("Sum: {:e}", sum);
        calc_rel_error(expected, sum);

        println!("Kahan sum for {}n: ", i);
        let sum = basel_sum_kahal(n*i);
        println!("Sum: {:e}", sum);
        calc_rel_error(expected, sum);
    }
}

fn main() {
    println!("--------------------------- Task 1 ---------------------------");
    task_1();
    println!("--------------------------- Task 2 ---------------------------");
    task_2();
    println!("--------------------------- Task 3 ---------------------------");
    task_3();
}
