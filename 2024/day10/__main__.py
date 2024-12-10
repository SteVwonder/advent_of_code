import os
import argparse

import itertools
import networkx as nx
import matplotlib.pyplot as plt

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

Coords = tuple[int, int]

class Map:
    def __init__(self, graph, starting_points, ending_points):
        self.graph = graph
        self.starting_points = set(starting_points)
        self.ending_points = set(ending_points)

    @staticmethod
    def check_for_edge(matrix, coords_a, coords_b) -> tuple[Coords, Coords] | None:
        val_a = matrix[coords_a[0]][coords_a[1]]
        val_b = matrix[coords_b[0]][coords_b[1]]
        if abs(val_a - val_b) == 1:
            if val_a < val_b:
                return (coords_a, coords_b)
            elif val_b < val_a:
                return (coords_b, coords_a)
        return None

    @staticmethod
    def from_file(input_file):
        matrix = [[int(x) for x in line.rstrip()] for line in get_lines(input_file)]
        starting_points = []
        ending_points = []
        graph = nx.DiGraph()
        for row_idx, row in enumerate(matrix):
            for col_idx, val in enumerate(row):
                coords = (row_idx, col_idx)
                graph.add_node(coords, height=val)
                if val == 0:
                    starting_points.append(coords)
                elif val == 9:
                    ending_points.append(coords)

                edges_to_check = []
                if row_idx > 0:
                    edges_to_check.append((row_idx-1, col_idx))
                if col_idx > 0:
                    edges_to_check.append((row_idx, col_idx-1))
                for other_coords in edges_to_check:
                    edge = Map.check_for_edge(matrix, coords, other_coords)
                    if edge is not None:
                        graph.add_edge(*edge)
        return Map(graph, starting_points, ending_points)

    def visualize(self):
        # Position nodes based on their coordinates
        pos = {node: (node[1], -node[0]) for node in self.graph.nodes()}

        # Extract node labels (heights)
        labels = nx.get_node_attributes(self.graph, 'height')

        # Create a list of node colors
        node_colors = ['lightblue' for _ in self.graph.nodes()]
        for i, node in enumerate(self.graph.nodes()):
            if node in self.starting_points:
                node_colors[i] = 'red'
            elif node in self.ending_points:
                node_colors[i] = 'blue'

        # Draw the graph
        plt.figure(figsize=(10, 10))

        # Add a legend
        red_patch = plt.Circle((0, 0), 0.1, fc="red")
        blue_patch = plt.Circle((0, 0), 0.1, fc="blue")
        lightblue_patch = plt.Circle((0, 0), 0.1, fc="lightblue")
        plt.legend([red_patch, blue_patch, lightblue_patch], 
                   ['Starting Points', 'Ending Points', 'Other Points'], 
                   loc='upper left', bbox_to_anchor=(1, 1))

        nx.draw(self.graph,
                pos,
                with_labels=True,
                labels=labels,
                node_size=500,
                node_color=node_colors,
                font_size=10,
                font_color="black",
                arrowsize=20)

        plt.tight_layout()
        plt.show()

def part1(map):
    shortest_paths = nx.all_pairs_shortest_path_length(map.graph)
    answer = 0
    for (source, targets) in shortest_paths:
        if source in map.starting_points:
            for ending_point in map.ending_points:
                if ending_point in targets:
                    answer += 1
    return answer

def part2(map):
    answer = 0
    for (source, target) in itertools.product(map.starting_points, map.ending_points):
        for _ in nx.all_simple_paths(map.graph, source, target):
            answer += 1
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
