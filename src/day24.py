import sys
from collections import deque
import numpy as np


class Hailstone:
    x: int
    y: int
    z: int

    a: int
    b: int
    c: int

    def __init__(self, x, y, z, a, b, c):
        self.x = x
        self.y = y
        self.z = z
        self.a = a
        self.b = b
        self.c = c

    def __str__(self):
        return f"Line ({self.x}, {self.y}) + ({self.a}, {self.b})"


def parametric_to_cartesian(point, direction):
    x0, y0 = point
    vx, vy = direction

    if vx == 0:  # The line is vertical
        # print(f"The line is vertical with equation x = {x0}")
        return None  # Cartesian form is not a function of y

    # Calculate the slope (m) and y-intercept (b)
    m = vy / vx
    b = y0 - m * x0

    return m, b


def determine_time_of_intersection(point, direction, intersection):
    # print(f"intersection: {intersection}")
    x0, y0 = point
    vx, vy = direction
    x_int, y_int = intersection

    # Choose the equation with a non-zero direction vector component
    if vx != 0:
        t = (x_int - x0) / vx
    elif vy != 0:
        t = (y_int - y0) / vy
    else:
        raise ValueError("The direction vector cannot be zero.")

    # Determine if the intersection is in the past or future
    if t > 0:
        return t, "future"
    elif t < 0:
        return t, "past"
    else:
        return t, "present"


def line_intersection_cartesian(point1, dir1, point2, dir2):
    m1, b1 = parametric_to_cartesian(point1, dir1)
    m2, b2 = parametric_to_cartesian(point2, dir2)

    a = m1
    c = b1
    b = m2
    d = b2

    if a - b == 0:
        return None

    x = (d - c) / (a - b)
    y = a * (d - c) / (a - b) + c

    time1 = determine_time_of_intersection(point1, dir1, (x, y))
    time2 = determine_time_of_intersection(point2, dir2, (x, y))

    if time1[1] == "past" or time2[1] == "past":
        return None
    else:
        return x, y


def find_intersection(hailstone1, hailstone2):
    # print(f"Finding intersection between {hailstone1} and {hailstone2}")
    return line_intersection_cartesian((hailstone1.x, hailstone1.y), (hailstone1.a, hailstone1.b),
                                       (hailstone2.x, hailstone2.y),
                                       (hailstone2.a, hailstone2.b))


def point_inside_rectangle(point, rectangle):
    (x1, y1), (x2, y2) = rectangle
    px, py = point

    return x1 <= px <= x2 and y1 <= py <= y2


def solve_part_1(lines):
    hailstones = []
    for line in lines:
        parts = line.split("@")
        coords = parts[0].split(",")
        x = int(coords[0].strip())
        y = int(coords[1].strip())
        z = int(coords[2].strip())

        dir = parts[1].split(",")
        a = int(dir[0].strip())
        b = int(dir[1].strip())
        c = int(dir[2].strip())

        hailstones.append(Hailstone(x, y, z, a, b, c))

    intersections = 0
    for index, hailstone1 in enumerate(hailstones):
        for hailstone2 in hailstones[index + 1:]:
            if hailstone1 != hailstone2:
                intersection = find_intersection(hailstone1, hailstone2)
                # print(f"Intersection: {intersection}")
                if intersection is not None:
                    if point_inside_rectangle(intersection, ((200000000000000, 200000000000000), (400000000000000, 400000000000000))):
                    # if point_inside_rectangle(intersection, ((7, 7), (27, 27))):
                        intersections += 1
                        # print("Inside!")
    return intersections

def solve_part_2(lines):
    # Define the coefficient matrix A and the constant vector b
    hailstones = []
    for line in lines:
        parts = line.split("@")
        coords = parts[0].split(",")
        x = int(coords[0].strip())
        y = int(coords[1].strip())
        z = int(coords[2].strip())

        dir = parts[1].split(",")
        a = int(dir[0].strip())
        b = int(dir[1].strip())
        c = int(dir[2].strip())

        hailstones.append(Hailstone(x, y, z, a, b, c))

    px0 = hailstones[0].x
    py0 = hailstones[0].y
    pz0 = hailstones[0].z

    vx0 = hailstones[0].a
    vy0 = hailstones[0].b
    vz0 = hailstones[0].c

    px1 = hailstones[1].x
    py1 = hailstones[1].y
    pz1 = hailstones[1].z

    vx1 = hailstones[1].a
    vy1 = hailstones[1].b
    vz1 = hailstones[1].c

    px2 = hailstones[2].x
    py2 = hailstones[2].y
    pz2 = hailstones[2].z

    vx2 = hailstones[2].a
    vy2 = hailstones[2].b
    vz2 = hailstones[2].c

    px3 = hailstones[3].x
    py3 = hailstones[3].y
    pz3 = hailstones[3].z

    vx3 = hailstones[3].a
    vy3 = hailstones[3].b
    vz3 = hailstones[3].c

    A = np.array([[vy0 - vy1, vx1 - vx0, 0        , py1 - py0, px0 - px1, 0        ],
                  [vz0 - vz1, 0        , vx1 - vx0, pz1 - pz0, 0        , px0 - px1],
                  [vy0 - vy2, vx2 - vx0, 0        , py2 - py0, px0 - px2, 0        ],
                  [vz0 - vz2, 0        , vx2 - vx0, pz2 - pz0, 0        , px0 - px2],
                  [vy0 - vy3, vx3 - vx0, 0        , py3 - py0, px0 - px3, 0        ],
                  [vz0 - vz3, 0        , vx3 - vx0, pz3 - pz0, 0        , px0 - px3]])
    b = np.array([px0*vy0 - py0*vx0 - px1*vy1 + py1*vx1,
                  px0*vz0 - pz0*vx0 - px1*vz1 + pz1*vx1,
                  px0*vy0 - py0*vx0 - px2*vy2 + py2*vx2,
                  px0*vz0 - pz0*vx0 - px2*vz2 + pz2*vx2,
                  px0*vy0 - py0*vx0 - px3*vy3 + py3*vx3,
                  px0*vz0 - pz0*vx0 - px3*vz3 + pz3*vx3])

    # Solve the system of equations
    x = np.linalg.solve(A, b)
    solution = x[0] + x[1] + x[2]
    print(f"Part 2: {solution}")


def solve():
    lines = open('input24.txt', 'r').readlines()

    result = solve_part_1(lines)
    print(f"Part 1: {result}")

    solve_part_2(lines)


solve()
