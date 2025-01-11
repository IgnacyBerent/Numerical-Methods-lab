import numpy as np
from functools import cache
from scipy.integrate import solve_ivp
from animate_func import animate_particles

initial_conditions = [
    {
        "name": "example 1",
        "m1": 1,
        "x1": -3,
        "y1": 0,
        "vx1": 1,
        "vy1": 0,
        "m2": 1,
        "x2": 3,
        "y2": 0,
        "vx2": -1,
        "vy2": 0,
    },
    {
        "name": "example 2",
        "m1": 1,
        "x1": -3,
        "y1": 1,
        "vx1": 1,
        "vy1": 0,
        "m2": 1,
        "x2": 3,
        "y2": 0,
        "vx2": -1,
        "vy2": 0,
    },
]


@cache
def lenard_jones_potential(r: float) -> float:
    return 24 * (2 / r**13 - 1 / r**7)


def system(t, y, m1, m2):
    x1, y1, x2, y2, vx1, vy1, vx2, vy2 = y
    r = np.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2)
    a = lenard_jones_potential(r)
    a1 = a / m1
    a2 = a / m2
    ax1 = (x2 - x1) * a1
    ay1 = (y2 - y1) * a1
    ax2 = (x1 - x2) * a2
    ay2 = (y1 - y2) * a2
    return [vx1, vy1, vx2, vy2, ax1, ay1, ax2, ay2]


if __name__ == "__main__":
    for ic in initial_conditions:
        y0 = [
            ic["x1"],
            ic["y1"],
            ic["x2"],
            ic["y2"],
            ic["vx1"],
            ic["vy1"],
            ic["vx2"],
            ic["vy2"],
        ]
        t_end = 10
        t_span = [0, 10]
        t_eval = np.linspace(0, t_end, 100)
        solution = solve_ivp(
            system,
            t_span,
            y0,
            t_eval=t_eval,
            args=(ic["m1"], ic["m2"]),
        )
        trajectory_1 = np.array([solution.y[0], solution.y[1]]).T
        trajectory_2 = np.array([solution.y[2], solution.y[3]]).T
        print(trajectory_1)
        anim = animate_particles(
            [trajectory_1, trajectory_2],
            solution.t,
            ic["name"],
            "x(t)",
            "y(t",
        )
