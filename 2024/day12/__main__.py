import os
import argparse

from itertools import chain, product
import networkx as nx
import matplotlib.pyplot as plt

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

Coords = tuple[int, int]

class Map:
    def __init__(self, graph):
        self.graph = graph

    @staticmethod
    def check_for_edge(matrix, coords_a, coords_b) -> tuple[Coords, Coords] | None:
        val_a = matrix[coords_a[0]][coords_a[1]]
        val_b = matrix[coords_b[0]][coords_b[1]]
        if val_a == val_b:
            return (coords_a, coords_b)
        else:
            return None

    @staticmethod
    def from_file(input_file):
        matrix = [[x for x in line.rstrip()] for line in get_lines(input_file)]
        graph = nx.Graph()
        for row_idx, row in enumerate(matrix):
            for col_idx, val in enumerate(row):
                coords = (row_idx, col_idx)
                graph.add_node(coords, crop=val)

                edges_to_check = []
                if row_idx > 0:
                    edges_to_check.append((row_idx-1, col_idx))
                if col_idx > 0:
                    edges_to_check.append((row_idx, col_idx-1))
                for other_coords in edges_to_check:
                    edge = Map.check_for_edge(matrix, coords, other_coords)
                    if edge is not None:
                        graph.add_edge(*edge)
        return Map(graph)

    def visualize(self):
        # Position nodes based on their coordinates
        pos = {node: (node[1], -node[0]) for node in self.graph.nodes()}

        labels = nx.get_node_attributes(self.graph, 'crop')

        # Draw the graph
        plt.figure(figsize=(10, 10))

        nx.draw(self.graph,
                pos,
                with_labels=True,
                labels=labels,
                node_size=500,
                node_color='lightblue',
                font_size=10,
                font_color="black",
                arrowsize=20)

        plt.tight_layout()
        plt.show()

def part1(map):
    regions = nx.connected_components(map.graph)
    answer = 0
    for region in regions:
        area = len(region)
        perimeter = sum([4 - map.graph.degree(node) for node in region])
        answer += area * perimeter
    return answer

def add_tuple(a, b):
     return (a[0] + b[0],
             a[1] + b[1])

def is_same_crop(graph, node, neighbor):
    node_crop = graph.nodes[node]['crop']
    try:
        neighbor_crop = graph.nodes[neighbor]['crop']
    except KeyError:
        return False
    return node_crop == neighbor_crop

def calculate_sides(graph, region):
    region_set = set(region)
    # North, South, East, West
    cardinal_directions = [(-1, 0), (1, 0), (0,1), (0, -1)]
    diagonals = [add_tuple(a, b) for (a,b) in product(cardinal_directions[0:2], cardinal_directions[2:])]
    sides = 0
    for node in region:
        neighbors = set(add_tuple(node, direction) for direction in chain(cardinal_directions, diagonals))
        neighbor_in_region = region_set.intersection(neighbors)

        for (dir_a, dir_b) in product(cardinal_directions[0:2], cardinal_directions[2:]):
            neighbor_a, neighbor_b = (add_tuple(node, dir_a), add_tuple(node, dir_b))
            # Outside Corners
            if neighbor_a not in neighbor_in_region and neighbor_b not in neighbor_in_region:
                sides += 1
            # Inside Corners
            diagonal_dir = add_tuple(dir_a, dir_b)
            diagonal_neighbor = add_tuple(node, diagonal_dir)
            if neighbor_a in neighbor_in_region and neighbor_b in neighbor_in_region and diagonal_neighbor not in neighbor_in_region:
                sides += 1

        # Inside Corners
    return sides

def part2(map):
    regions = nx.connected_components(map.graph)
    answer = 0
    for region in regions:
        area = len(region)
        crop = map.graph.nodes[next(iter(region))]['crop']
        sides = calculate_sides(map.graph, region)
        answer += area * sides
    return answer

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--test', action='store_true')
    parser.add_argument('--visualize', action='store_true')
    args = parser.parse_args()

    input_file = 'input'
    if args.test:
        input_file = 'test'
    input_file = os.path.join(os.path.dirname(os.path.abspath(__file__)), input_file)
    map = Map.from_file(input_file)
    if args.visualize:
        map.visualize()
    print(part1(map))
    print(part2(map))

if __name__ == "__main__":
    main()
