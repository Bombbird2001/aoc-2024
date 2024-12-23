from collections import defaultdict
import networkx as nx
from networkx.algorithms.clique import find_cliques

adj_list = defaultdict(set)


G = nx.Graph()
with open("input.txt", 'r') as f:
    for line in f.read().split('\n'):
        node_1 = line.split('-')[0]
        node_2 = line.split('-')[1]
        adj_list[node_1].add(node_2)
        adj_list[node_2].add(node_1)
        G.add_edge(node_1, node_2)


# Part 1
connected = set()
for node_key in adj_list:
    if not node_key.startswith('t'):
        continue
    neighbour_list = list(adj_list[node_key])
    for i, n1 in enumerate(neighbour_list):
        for j in range(i + 1, len(neighbour_list)):
            n2 = neighbour_list[j]
            if n1 in adj_list[n2] and n2 in adj_list[n1]:
                connected.add(frozenset((node_key, n1, n2)))


print(len(connected))


# Part 2
max_clique = max([clique for clique in find_cliques(G)], key=len)
print(','.join(sorted(max_clique)))