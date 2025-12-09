#!/usr/bin/env python3
"""
Visualize polygon from coordinate file.
Reads x,y coordinates from data/inputs/09.txt and displays them as a connected polygon.
"""

import matplotlib.pyplot as plt

def read_coordinates(filename):
    """Read coordinates from file and return lists of x and y values."""
    x_coords = []
    y_coords = []

    with open(filename, 'r') as f:
        for line in f:
            line = line.strip()
            if line:
                x, y = map(int, line.split(','))
                x_coords.append(x)
                y_coords.append(y)

    return x_coords, y_coords

def plot_polygon(x_coords, y_coords):
    """Plot the polygon with lines connecting all points."""
    # Create figure and axis
    fig, ax = plt.subplots(figsize=(12, 10))

    # Close the polygon by adding first point at the end
    x_closed = x_coords + [x_coords[0]]
    y_closed = y_coords + [y_coords[0]]

    # Plot the polygon
    ax.plot(x_closed, y_closed, 'b-', linewidth=2, label='Polygon edges')
    ax.plot(x_coords, y_coords, 'ro', markersize=4, label='Vertices')

    # Mark the first point specially
    ax.plot(x_coords[0], y_coords[0], 'go', markersize=8, label='Start point')

    # Add additional rectangle with corners (17454, 85504) and (82409, 14643)
    rect_x = [17454, 82409, 82409, 17454, 17454]
    rect_y = [85504, 85504, 14643, 14643, 85504]
    ax.plot(rect_x, rect_y, 'r-', linewidth=2, label='Rectangle', alpha=0.7)

    # Set labels and title
    ax.set_xlabel('X coordinate')
    ax.set_ylabel('Y coordinate')
    ax.set_title(f'Polygon Visualization ({len(x_coords)} vertices)')
    ax.legend()
    ax.grid(True, alpha=0.3)
    ax.axis('equal')

    plt.tight_layout()
    plt.show()

if __name__ == '__main__':
    # Read coordinates
    x_coords, y_coords = read_coordinates('data/inputs/09.txt')

    print(f"Loaded {len(x_coords)} coordinates")
    print(f"X range: {min(x_coords)} to {max(x_coords)}")
    print(f"Y range: {min(y_coords)} to {max(y_coords)}")

    # Plot the polygon
    plot_polygon(x_coords, y_coords)
