import random
from animate import animate_simulation
from typing import List, Tuple, Callable
from functools import cache
import numpy as np
import matplotlib.pyplot as plt


class Particle:
    def __init__(self, x: float, y: float, vx: float, vy: float):
        self.position = np.array([x, y])
        self.velocity = np.array([vx, vy])
        self.forces = np.array([0.0, 0.0])
        self.half_velocity = np.array([0.0, 0.0])
        self.trajectories = np.array([self.position])
        self.velocities = np.array([self.velocity])
        self.total_energy_track = np.array([])
        self.current_energy_potential = 0
    
    def bounce(self, sys_boundaries: int):
        if self.position[0] < -sys_boundaries or self.position[0] > sys_boundaries:
            self.velocity[0] = -self.velocity[0]
        if self.position[1] < -sys_boundaries or self.position[1] > sys_boundaries:
            self.velocity[1] = -self.velocity[1]

    def update_forces(self, particles: List["Particle"]):
        # Reset forces
        self.forces = np.array([0.0, 0.0])
        # reset energy
        self.current_energy_potential = 0
        # Calculate forces from other particles
        for particle in particles:
            if particle is not self:
                d = self.position - particle.position
                r = np.linalg.norm(d)
                self.forces += self.lennard_jones_force(d, r)
                self.current_energy_potential += self.lennard_jones_potential(r)
        
    @staticmethod
    def lennard_jones_force(d, r: float, rm: float = 1.0, epsilon: float = 0.1) -> float:
        return 12 * d / (rm * r) * epsilon * ((rm / r) ** 13 - (rm / r) ** 7)
    
    @staticmethod
    def lennard_jones_potential(r, rm=1.0, epsilon=0.1):
        return epsilon * ((rm / r) ** 12 - 2 * (rm / r) ** 6)
    

    def calculate_half_step_velocity(self, dt: float):
        self.half_velocity = self.velocity + self.forces * dt / 2
    
    def update_position(self, dt: float):
        self.position = self.position + self.half_velocity * dt 
        self.trajectories = np.append(self.trajectories, [self.position], axis=0)

    def update_velocity(self, dt: float):
        self.velocity = self.half_velocity + self.forces * dt / 2
        self.velocities = np.append(self.velocities, [self.velocity], axis=0)
        self.total_energy_track = np.append(self.total_energy_track, [self.calc_kinetic_energy() + self.current_energy_potential], axis=0)

    def calc_kinetic_energy(self):
        return 0.5 * np.linalg.norm(self.velocity) ** 2
    



def init_particles(n_of_particles, pos_range, v_range, seed=None):
    particles = []
    if seed:
        random.seed(seed)
    for _ in range(n_of_particles):
        x = random.uniform(*pos_range)
        y = random.uniform(*pos_range)
        vx = random.choice([*v_range])
        vy = random.choice([*v_range])
        particles.append(Particle(x, y, vx, vy))
    
    # init forces
    for particle in particles:
        particle.update_forces(particles)

    return particles


def simulation(particles, dt, t, sys_boundaries, title):
    tcurr = 0
    while tcurr < t:
        # calculate all half velocities
        for particle in particles:
            particle.calculate_half_step_velocity(dt)
        # update all positions
        for particle in particles:
            particle.update_position(dt)
        # update all forces
        # update all velocities
        # bounce all particles
        for particle in particles:
            particle.update_forces(particles)
            particle.update_velocity(dt)
            particle.bounce(sys_boundaries)

        tcurr += dt
        

    trajectories = [particle.trajectories for particle in particles]
    animate_simulation(trajectories, sys_boundaries, np.arange(0, t), title)

    # calculate total energy of the system over time and plot it
    total_energy = np.zeros(len(particles[0].total_energy_track))
    for particle in particles:
        total_energy += particle.total_energy_track

    plt.plot(total_energy)
    plt.title('Total energy of the system over time')
    plt.xlabel('Time')
    plt.ylabel('Energy')
    plt.show()


def test_1():
    sys_boundaries = 5
    seed = 42
    dt = 0.01
    t = 200
    rx = 4
    ry = 0
    vx = 3
    
    particles = [Particle(-rx/2, -ry/2, vx, 0), Particle(rx/2, ry/2, -vx, 0)]
    for particle in particles:
        particle.update_forces(particles)
    simulation(particles, dt, t, sys_boundaries, 'Test two particles with opposite velocities')


def test_2():
    sys_boundaries = 5
    seed = 42
    dt = 0.01
    t = 200
    rx = 3
    ry = 1.05
    vx = 3
    
    particles = [Particle(-rx/2, -ry/2, vx, 0), Particle(rx/2, ry/2, -vx, 0)]
    for particle in particles:
        particle.update_forces(particles)
    simulation(particles, dt, t, sys_boundaries, 'Test two particles with opposite velocities and shifted in y axis by more than rm')


def test_3():
    sys_boundaries = 5
    seed = 42
    dt = 0.01
    t = 100
    rx = 3
    ry = 0.8
    vx = 3
    
    particles = [Particle(-rx/2, -ry/2, vx, 0), Particle(rx/2, ry/2, -vx, 0)]
    for particle in particles:
        particle.update_forces(particles)
    simulation(particles, dt, t, sys_boundaries, 'Test two particles with opposite velocities and shifted in y axis by less than rm')


def test_4():
    sys_boundaries = 5
    seed = 42
    dt = 0.01
    t = 100
    rx = 3
    ry = 0
    vx = 2
    vy = 1
    
    particles = [Particle(-rx/2, -ry/2, vx, vy), Particle(rx/2, ry/2, -vx, vy)]
    for particle in particles:
        particle.update_forces(particles)
    simulation(particles, dt, t, sys_boundaries, 'Test two particles with opposite velocities going up')


def sim():
    n_of_particles = 15
    pos_range = (-5, 5)
    v_range = (-2, 2)
    sys_boundaries = 5
    seed = 42
    dt = 0.01
    t = 300
    
    particles = init_particles(n_of_particles, pos_range, v_range, seed)
    simulation(particles, dt, t, sys_boundaries, 'Simulation of 15 particles')
                

if __name__ == "__main__":
    TEST = False
    if TEST:
        test_1()
        test_2()
        test_3()
        test_4()
    else:
        sim()
    