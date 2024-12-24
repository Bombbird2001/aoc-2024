import graphviz


dot = graphviz.Digraph(comment="Advent of Code 2024 - Day 24 part 2")
all_nodes = set()
for i in range(45):
    if i >= 10:
        dot.node(f'x{i}', f'x{i}')
        dot.node(f'y{i}', f'y{i}')
        dot.node(f'z{i}', f'z{i}')
        all_nodes.add(f'x{i}')
        all_nodes.add(f'y{i}')
        all_nodes.add(f'z{i}')
    else:
        dot.node(f'x0{i}', f'x0{i}')
        dot.node(f'y0{i}', f'y0{i}')
        dot.node(f'z0{i}', f'z0{i}')
        all_nodes.add(f'x0{i}')
        all_nodes.add(f'y0{i}')
        all_nodes.add(f'z0{i}')
dot.node('z45', 'z45')
all_nodes.add('z45')


with open('target/debug/input.txt', 'r') as f:
    lines = f.read().split('\n')
    node_mode = False
    for line in lines:
        if len(line) == 0:
            node_mode = True
            continue
        if node_mode:
            nodes = line.strip().split(' ')
            if nodes[0] not in all_nodes:
                all_nodes.add(nodes[0])
                dot.node(nodes[0], nodes[0])
            if nodes[2] not in all_nodes:
                all_nodes.add(nodes[2])
                dot.node(nodes[2], nodes[2])
            if nodes[4] not in all_nodes:
                all_nodes.add(nodes[4])
                dot.node(nodes[4], nodes[4])
            dot.edge(nodes[0], nodes[4])
            dot.edge(nodes[2], nodes[4])


# dot.render('d24b', format='png', cleanup=True)


print(','.join(sorted(['z09', 'nnf', 'z20', 'nhs', 'kqh', 'ddn', 'z34', 'wrc'])))