from timeit import timeit
import math

def solution():
    sol = 0.0
    k = 1.0
    check = 0.0
    while True:
        partial_sol = 1/k**2
        sol += partial_sol
        if sol == check:
            return sol, k
        check = sol
        k += 1

def main():
    print("Python solution:")
    approx, k = solution()
    print('final approx:', approx)
    print('n_max:', k)
    print(f'time: {timeit(lambda: solution(), number=1)}')
    print('percent error:', abs(approx - math.pi**2/6)/(math.pi**2/6)*100)

if __name__ == "__main__":
    main()
