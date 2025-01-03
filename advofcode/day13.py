import re
import numpy as np
from timeit import timeit

pattern = r"X\+(\d+), Y\+(\d+)"
pattern_result = r"X\=(\d+), Y\=(\d+)"

a_values = []
b_values = []
reward_values = []


def read_input():
    with open("input.txt") as f:
        lines = f.readlines()
    for i, line in enumerate(lines):
        if i % 4 == 0:
            matches = re.search(pattern, line)
            a_values.append((int(matches.group(1)), int(matches.group(2))))
        elif i % 4 == 1:
            matches = re.search(pattern, line)
            b_values.append((int(matches.group(1)), int(matches.group(2))))
        elif i % 4 == 2:
            matches = re.search(pattern_result, line)
            reward_values.append((int(matches.group(1)), int(matches.group(2))))


def calculate_cost(a_clicks, b_clicks):
    return a_clicks * 3 + b_clicks * 1


def solve_equation(r, a, b):
    b_clicks = (r[0] * a[1] - r[1] * a[0]) / (b[0] * a[1] - b[1] * a[0])
    a_clicks = (r[0] - b_clicks * b[0]) / a[0]
    return a_clicks, b_clicks


def check_if_int(a_clicks, b_clicks):
    return a_clicks.is_integer() and b_clicks.is_integer()


def main():
    read_input()
    clicks = []
    clicks2 = []
    for i in range(len(a_values)):
        r_new = (
            reward_values[i][0] + 10000000000000,
            reward_values[i][1] + 10000000000000,
        )
        a_clicks, b_clicks = solve_equation(reward_values[i], a_values[i], b_values[i])
        a_clicks2, b_clicks2 = solve_equation(r_new, a_values[i], b_values[i])
        if check_if_int(a_clicks, b_clicks):
            clicks.append((a_clicks, b_clicks))
        if check_if_int(a_clicks2, b_clicks2):
            clicks2.append((a_clicks, b_clicks))
    cost_1 = sum(
        [calculate_cost(cs[0], cs[1]) for cs in clicks if cs[0] <= 100 and cs[1] <= 100]
    )
    cost_2 = sum([calculate_cost(cs[0], cs[1]) for cs in clicks2])

    print(int(cost_1), int(cost_2))


if __name__ == "__main__":
    main()
    time = timeit(lambda: main, number=1)
    print(f"Time: {time}")
