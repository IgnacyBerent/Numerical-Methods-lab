import numpy as np
from functools import cache
from scipy.integrate import solve_ivp
from animate_func import animate_particles

G = 1

initial_conditions = [
    {
        "name": "Figure 8 solution",
        "m1": 1.0,
        "m2": 1.0,
        "m3": 1.0,
        "x1": -0.97000436,
        "y1": 0.24308753,
        "vx1": 0.466203685,
        "vy1": 0.43236573,
        "x2": 0,
        "y2": 0,
        "vx2": -0.93240737,
        "vy2": -0.86473146,
        "x3": 0.97000436,
        "y3": -0.24308753,
        "vx3": 0.466203685,
        "vy3": 0.43236573,
    },
    {
        "name": "Euler's solution",
        "m1": 1.0,
        "m2": 4.0,
        "m3": 1.0,
        "x1": -1,
        "y1": 0,
        "vx1": 0,
        "vy1": 0.3,
        "x2": 0,
        "y2": 0,
        "vx2": 0,
        "vy2": -0.6,
        "x3": 1,
        "y3": 0,
        "vx3": 0,
        "vy3": 0.3,
    },
    {
        "name": "Lagrange Equilateral Triangle solution",
        "m1": 1.0,
        "m2": 1.0,
        "m3": 1.0,
        "x1": -1,
        "y1": 0,
        "vx1": 0,
        "vy1": -0.5,
        "x2": 0.5,
        "y2": 0.866,
        "vx2": -0.433,
        "vy2": -0.25,
        "x3": 0.5,
        "y3": -0.866,
        "vx3": 0.433,
        "vy3": -0.25,
    },
    {
        "name": "Stinkov solution",
        "m1": 1.0,
        "m2": 1.0,
        "m3": 0.1,
        "x1": -0.5,
        "y1": 0,
        "vx1": 0,
        "vy1": -0.6,
        "x2": 0.5,
        "y2": 0,
        "vx2": 0,
        "vy2": 0.6,
        "x3": 0,
        "y3": 0,
        "vx3": 0.3,
        "vy3": 0,
    },
]


def acceleration(x1: float, x2: float, m2: float, r: float) -> float:
    return G * m2 * (x2 - x1) / r**3


def system(t, y, m1, m2, m3):
    x1, y1, x2, y2, x3, y3, vx1, vy1, vx2, vy2, vx3, vy3 = y

    r12 = np.sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2)
    r13 = np.sqrt((x3 - x1) ** 2 + (y3 - y1) ** 2)
    r23 = np.sqrt((x3 - x2) ** 2 + (y3 - y2) ** 2)

    ax1 = acceleration(x1, x2, m2, r12) + acceleration(x1, x3, m3, r13)
    ay1 = acceleration(y1, y2, m2, r12) + acceleration(y1, y3, m3, r13)
    ax2 = acceleration(x2, x1, m1, r12) + acceleration(x2, x3, m3, r23)
    ay2 = acceleration(y2, y1, m1, r12) + acceleration(y2, y3, m3, r23)
    ax3 = acceleration(x3, x1, m1, r13) + acceleration(x3, x2, m2, r23)
    ay3 = acceleration(y3, y1, m1, r13) + acceleration(y3, y2, m2, r23)

    return [vx1, vy1, vx2, vy2, vx3, vy3, ax1, ay1, ax2, ay2, ax3, ay3]


if __name__ == "__main__":
    for ic in initial_conditions:
        y0 = [
            ic["x1"],
            ic["y1"],
            ic["x2"],
            ic["y2"],
            ic["x3"],
            ic["y3"],
            ic["vx1"],
            ic["vy1"],
            ic["vx2"],
            ic["vy2"],
            ic["vx3"],
            ic["vy3"],
        ]

        m1, m2, m3 = ic["m1"], ic["m2"], ic["m3"]

        t_end = 10
        t_span = [0, 10]
        t_eval = np.linspace(0, t_end, 100)
        solution = solve_ivp(system, t_span, y0, t_eval=t_eval, args=(m1, m2, m3))
        trajectory_1 = np.array([solution.y[0], solution.y[1]]).T
        trajectory_2 = np.array([solution.y[2], solution.y[3]]).T
        trajectory_3 = np.array([solution.y[4], solution.y[5]]).T
        anim = animate_particles(
            [trajectory_1, trajectory_2, trajectory_3],
            solution.t,
            ic["name"],
            "x(t)",
            "y(t",
        )
