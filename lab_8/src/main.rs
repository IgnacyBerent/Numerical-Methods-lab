struct Equation {
    x1: f64,
    x2: f64,
    x3: f64,
    r: f64,
}

impl Equation {
    fn new(x1: f64, x2: f64, x3: f64, r: f64) -> Equation {
        Equation { x1, x2, x3, r }
    }

    fn multiply(&self, value: f64) -> Equation {
        Equation {
            x1: self.x1 * value,
            x2: self.x2 * value,
            x3: self.x3 * value,
            r: self.r * value,
        }
    }

    fn divide(&self, value: f64) -> Equation {
        Equation {
            x1: self.x1 / value,
            x2: self.x2 / value,
            x3: self.x3 / value,
            r: self.r / value,
        }
    }

    fn add(&mut self, other: &Equation) -> Equation {
        Equation {
            x1: self.x1 + other.x1,
            x2: self.x2 + other.x2,
            x3: self.x3 + other.x3,
            r: self.r + other.r,
        }
    }

    fn subtract(&mut self, other: &Equation) -> Equation {
        Equation {
            x1: self.x1 - other.x1,
            x2: self.x2 - other.x2,
            x3: self.x3 - other.x3,
            r: self.r - other.r,
        }
    }

    fn print(&self) {
        println!("{}x1 + {}x2 + {}x3 = {}", self.x1, self.x2, self.x3, self.r);
    }
}

struct EquationSystem {
    eq1: Equation,
    eq2: Equation,
    eq3: Equation,
}

impl EquationSystem {
    fn new(eq1: Equation, eq2: Equation, eq3: Equation) -> EquationSystem {
        EquationSystem { eq1, eq2, eq3 }
    }

    fn print(&self) {
        self.eq1.print();
        self.eq2.print();
        self.eq3.print();
    }

    fn solve_naive_gauss_el(&mut self) -> (f64, f64, f64) {
        // Divide the first equation by x1 to get 1 as the coefficient of x1
        let eq1 = self.eq1.divide(self.eq1.x1);

        // Subtract the first equation multiplied by x1 from the second and third equations
        // to get 0 as the coefficient of x1
        let mut eq2 = self.eq2.subtract(&eq1.multiply(self.eq2.x1));
        let mut eq3 = self.eq3.subtract(&eq1.multiply(self.eq3.x1));

        // Divide the second equation by x2 to get 1 as the coefficient of x2
        eq2 = eq2.divide(eq2.x2);

        // Subtract the second equation multiplied by x2 coefficient from the third equation
        // to get 0 as the coefficient of x2
        eq3 = eq3.subtract(&eq2.multiply(eq3.x2));

        let x3 = eq3.r / eq3.x3;
        let x2 = (eq2.r - eq2.x3 * x3) / eq2.x2;
        let x1 = (eq1.r - eq1.x2 * x2 - eq1.x3 * x3) / eq1.x1;

        return (x1, x2, x3);
    }

    fn check_solution(&self, x1: f64, x2: f64, x3: f64) {
        let eq1 = self.eq1.x1 * x1 + self.eq1.x2 * x2 + self.eq1.x3 * x3;
        let eq2 = self.eq2.x1 * x1 + self.eq2.x2 * x2 + self.eq2.x3 * x3;
        let eq3 = self.eq3.x1 * x1 + self.eq3.x2 * x2 + self.eq3.x3 * x3;

        let is_correct = eq1 == self.eq1.r && eq2 == self.eq2.r && eq3 == self.eq3.r;

        if is_correct {
            println!("The solution is correct.");
        } else {
            println!("The solution is incorrect.");
        }
    }
}

fn main() {
    let eq1 = Equation::new(10.0, 2.0, -1.0, 27.0);
    let eq2 = Equation::new(-3.0, -6.0, 2.0, -61.5);
    let eq3 = Equation::new(1.0, 1.0, 5.0, -21.5);

    let mut eq_sys = EquationSystem::new(eq1, eq2, eq3);
    eq_sys.print();
    eq_sys.check_solution(0.5, 8.0, -6.0);

    let (x1, x2, x3) = eq_sys.solve_naive_gauss_el();
    print!("x1 = {}, x2 = {}, x3 = {}", x1, x2, x3);
}
