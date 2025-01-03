import numpy as np
from functools import cache
from scipy.integrate import solve_ivp
from animate_func import animate_particles

initial_conditions = {
    "x0": 2.0,
    "v0": 0.0,
    "m": 1.0,
}


@cache
def acceleration(x: float) -> float:
    return 24 * (2 / x**13 - 1 / x**7) / initial_conditions["m"]


def system(t, y):
    x, v = y
    return [v, acceleration(x)]


y0 = [initial_conditions["x0"], initial_conditions["v0"]]

t_end = 10
t_span = [0, 10]
t_eval = np.linspace(0, t_end, 100)
solution = solve_ivp(system, t_span, y0, t_eval=t_eval)

if __name__ == "__main__":
    anim = animate_particles(
        [solution.y.T], solution.t, "Motion for Lenard-Jones potential", "x(t)", "v(t)"
    )
