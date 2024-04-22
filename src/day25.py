import copy
import random


def contract(adj_lists: dict, u, v):
    # Merge v into u.
    while v in adj_lists[u]:
        adj_lists[u].remove(v)
    while u in adj_lists[v]:
        adj_lists[v].remove(u)
    for node in adj_lists[v]:
        adj_lists[u].append(node)
        adj_lists[node].remove(v)
        adj_lists[node].append(u)
    del adj_lists[v]


def find_min_cut_with_kagler(adj_lists: dict):
    component_sizes = {node: 1 for node in adj_lists}

    while len(adj_lists) > 2:
        u = random.choice(list(adj_lists.keys()))
        v = random.choice(adj_lists[u])
        contract(adj_lists, u, v)
        component_sizes[u] += component_sizes[v]
        del component_sizes[v]

    u = list(adj_lists.keys())[0]
    v = list(adj_lists.keys())[1]
    answer = component_sizes[u] * component_sizes[v]
    return len(adj_lists[u]), answer


def solve_part_1(lines: list[str]):
    nodes = set()
    node_to_linked_nodes = {}
    edges = set()
    start_node = None
    for line in lines:
        line = line.strip()
        parts = line.split(":")
        left = parts[0].strip()
        if start_node is None:
            start_node = left
        nodes.add(left)
        linked_nodes = parts[1].strip().split(" ")
        left_linked_node_list = node_to_linked_nodes.get(left, [])
        for right in linked_nodes:
            right_linked_node_list = node_to_linked_nodes.get(right, [])
            left_linked_node_list.append(right)
            right_linked_node_list.append(left)
            edges.add(f"{left}_{right}")
            nodes.add(right)
            node_to_linked_nodes[right] = right_linked_node_list
        node_to_linked_nodes[left] = left_linked_node_list

    min_cut = 0
    answer = 0
    while min_cut != 3:
        adj_list = copy.deepcopy(node_to_linked_nodes)
        min_cut, answer = find_min_cut_with_kagler(adj_list)
    return answer


def solve():
    lines = open('input25.txt', 'r').readlines()
    print("Part 1:", solve_part_1(lines))


solve()
