import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import random


def animate_particles(trajectories, t, title, xlabel, ylabel):
    n_of_particles = len(trajectories)
    colors_all = [
        "red",
        "blue",
        "green",
        "orange",
        "purple",
        "brown",
        "pink",
        "gray",
        "cyan",
        "magenta",
    ]

    # Create figure and axis
    fig, ax = plt.subplots(figsize=(8, 8))
    ax.set_xlim(-4, 4)
    ax.set_ylim(-4, 4)
    ax.set_aspect("equal")
    ax.grid(True)

    # Initialize lines for particles and trails
    colors = random.sample(colors_all, n_of_particles)
    particles = [
        ax.plot([], [], "o", color=c, markersize=10, label=f"Particle {i+1}")[0]
        for i, c in enumerate(colors)
    ]
    trails = [ax.plot([], [], "-", color=c, alpha=0.3)[0] for c in colors]

    ax.legend()
    plt.title(title)
    plt.xlabel(xlabel)
    plt.ylabel(ylabel)

    def init():
        """Initialize animation"""
        for particle, trail in zip(particles, trails):
            particle.set_data([], [])
            trail.set_data([], [])
        return particles + trails

    def update(frame):
        """Update animation"""
        trail_length = 50
        for i, (particle, trail) in enumerate(zip(particles, trails)):
            # Update particle position
            particle.set_data([trajectories[i][frame, 0]], [trajectories[i][frame, 1]])

            # Update trail
            start = max(0, frame - trail_length)
            trail.set_data(
                trajectories[i][start : frame + 1, 0],
                trajectories[i][start : frame + 1, 1],
            )
        return particles + trails

    # Create animation
    anim = FuncAnimation(
        fig, update, frames=len(t), init_func=init, blit=True, interval=20, repeat=True
    )

    filename = title.replace(" ", "_").lower()
    anim.save(f"{filename}.gif", writer="ffmpeg", fps=30)

    plt.show()
    return anim  # Keep a reference to prevent garbage collection
