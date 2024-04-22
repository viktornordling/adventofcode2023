import sys
from collections import deque

# Set a higher recursion limit
sys.setrecursionlimit(30000)


def print_grid(grid):
    minx = min(grid, key=lambda key: key[0])[0]
    maxx = max(grid, key=lambda key: key[0])[0]

    miny = min(grid, key=lambda key: key[1])[1]
    maxy = max(grid, key=lambda key: key[1])[1]

    for y in range(miny, maxy + 1):
        for x in range(minx, maxx + 1):
            print(grid[(x, y)], end='')
        print()


def solve_part_1(lines, goal_y):
    grid = {}
    for y, line in enumerate(lines):
        for x, s in enumerate(line.strip()):
            grid[(x, y)] = s

    dfs_seen = set()
    longest_path = 0

    def dfs(cur_pos, path):
        nonlocal longest_path
        # Do a DFS of the graph, but remove the current node from the seen nodes after we've explored
        # the subgraph, so that we find all possible paths, not just the first path that takes us to the
        # goal.
        if path is None:
            path = []
        dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        arrows = ['>', '<', '^', 'v']
        allowed_dir_for_arrow = {'>': (1, 0), '<': (-1, 0), '^': (0, -1), 'v': (0, 1)}
        if cur_pos[1] == goal_y:
            if len(path) > longest_path:
                longest_path = len(path)
        dfs_seen.add(cur_pos)
        path.append(cur_pos)
        for dir in dirs:
            new_pos = (cur_pos[0] + dir[0], cur_pos[1] + dir[1])
            neighbor = grid.get(new_pos, None)
            if neighbor is not None and new_pos not in dfs_seen and (
                    neighbor == '.' or (neighbor in arrows and dir == allowed_dir_for_arrow[neighbor])):
                dfs(new_pos, path)
        path.pop()
        dfs_seen.remove(cur_pos)

    dfs((1, 0), [])
    print(f"Part 1: {longest_path}")


def solve_part_2(lines, goal_y):
    grid = {}
    arrows = ['>', '<', '^', 'v']
    start_point = None
    end_point = None
    for y, line in enumerate(lines):
        for x, s in enumerate(line.strip()):
            if s in arrows:
                s = '.'
            grid[(x, y)] = s
            if y == 0 and s == '.':
                start_point = (x, y)
            elif y == len(lines) - 1 and s == '.':
                end_point = (x, y)

    seen = set()
    longest_path = 0

    adj_map = {}

    def create_adj_map():
        rows = len(lines)
        cols = len(lines[0])
        real_vertices = set()
        real_vertices.add(start_point)
        real_vertices.add(end_point)
        dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        for y in range(0, rows):
            for x in range(0, cols):
                cur_char = grid.get((x, y), None)
                if cur_char is not None and cur_char != '#':
                    cur_pos = (x, y)
                    num_possible_neighbors = 0
                    for dir in dirs:
                        new_pos = (cur_pos[0] + dir[0], cur_pos[1] + dir[1])
                        neighbor = grid.get(new_pos, None)
                        if neighbor is not None and neighbor != '#':
                            num_possible_neighbors += 1
                    if num_possible_neighbors > 2:
                        real_vertices.add(cur_pos)

        for vertex in real_vertices:
            bfs_queue = deque()
            bfs_queue.append((vertex, 0))
            seen = set()
            while len(bfs_queue) > 0:
                (cur, steps) = bfs_queue.pop()
                if cur not in seen:
                    seen.add(cur)
                    if cur in real_vertices and cur != vertex:
                        adj_set = adj_map.get(vertex, set())
                        adj_set.add((cur, steps))
                        adj_map[vertex] = adj_set
                    else:
                        for dir in dirs:
                            new_pos = (cur[0] + dir[0], cur[1] + dir[1])
                            neighbor = grid.get(new_pos, None)
                            if neighbor is not None and neighbor != '#' and new_pos not in seen:
                                tt = (new_pos, steps + 1)
                                bfs_queue.append(tt)

    def create_adjacency_map(cur_anchor, cur_pos, steps=0):
        # Do a DFS of the graph in order to build an adjacency map of "real choices" (a real choice
        # is a cell which actually has > 2 neighboring cells). So from each cell, connect it directly
        # to the next cell where a "real choice" occurs.
        dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        seen.add(cur_pos)
        if cur_pos[1] == goal_y:
            # If we get to the goal, add the goal as a connection to our current anchor.
            adj_set = adj_map.get(cur_anchor, set())
            adj_set.add((cur_pos, steps))
            # print(f"Appending {cur_pos} to {cur_anchor}, adj_set is {adj_set}")
            adj_map[cur_anchor] = adj_set
        num_possible_neighbors = 0
        for dir in dirs:
            new_pos = (cur_pos[0] + dir[0], cur_pos[1] + dir[1])
            neighbor = grid.get(new_pos, None)
            if neighbor is not None and neighbor != '#':
                num_possible_neighbors += 1
        new_anchor = cur_anchor
        next_steps = steps + 1
        if num_possible_neighbors > 2:
            # We found a real choice. Link the current anchor to this cell and then recurse from
            # this cell with this cell as the anchor.
            print(f"Found a real choice at pos {cur_pos} ({cur_pos[1] + 1}:{cur_pos[0] + 1})")
            adj_set = adj_map.get(cur_anchor, set())
            adj_set.add((cur_pos, steps))
            adj_map[cur_anchor] = adj_set
            new_anchor = cur_pos
            next_steps = 1
        for dir in dirs:
            new_pos = (cur_pos[0] + dir[0], cur_pos[1] + dir[1])
            neighbor = grid.get(new_pos, None)
            if neighbor is not None and neighbor != '#' and new_pos not in seen:
                create_adjacency_map(new_anchor, new_pos, next_steps)
        seen.remove(cur_pos)

    dfs_seen = set()

    def dfs(cur_pos, path, steps):
        nonlocal longest_path
        # Do a DFS of the graph, but remove the current node from the seen nodes after we've explored
        # the subgraph, so that we find all possible paths, not just the first path that takes us to the
        # goal.
        #
        # Each adjacency in the adjacency list contains the number of steps we have travelled, so
        # sum all those up to get the full path length.
        if path is None:
            path = []
        if cur_pos[1] == goal_y:
            if steps > longest_path:
                longest_path = steps
        dfs_seen.add(cur_pos)
        path.append(cur_pos)
        neighbors = adj_map.get(cur_pos, [])
        for pos_and_steps in neighbors:
            new_pos = pos_and_steps[0]
            neighbor = grid.get(new_pos, None)
            if neighbor is not None and new_pos not in dfs_seen and (neighbor != '#'):
                dfs(new_pos, path, steps + pos_and_steps[1])
        path.pop()
        dfs_seen.remove(cur_pos)

    create_adj_map()
    dfs((1, 0), [], 0)
    print(f"Part 2: {longest_path}")


def solve():
    lines = open('input23.txt', 'r').readlines()

    solve_part_1(lines, goal_y=len(lines) - 1)
    solve_part_2(lines, goal_y=len(lines) - 1)


solve()
