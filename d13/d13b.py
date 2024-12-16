import numpy as np
import re

button_regex = r".+X\+(\d+), Y\+(\d+)"
prize_regex = r".+X=(\d+), Y=(\d+)"

cost = 0

with open("target/debug/input.txt") as f:
    lines = f.readlines()
    for i in range(0, len(lines), 4):
        button_a = [int(x) for x in re.match(button_regex, lines[i]).groups()]
        button_b = [int(x) for x in re.match(button_regex, lines[i + 1]).groups()]
        # prize = [int(x) for x in re.match(prize_regex, lines[i + 2]).groups()]
        prize = [int(x) + 10000000000000 for x in re.match(prize_regex, lines[i + 2]).groups()]

        a = np.array([[button_a[0], button_b[0]],[button_a[1], button_b[1]]])
        b = np.array(prize)
        res = np.linalg.solve(a, b)
        # Check if the result is an integer
        if (abs(np.round(res) - res) < 0.01).all():
            # print(list(res))
            cost += np.sum(np.asarray([3, 1]) * np.round(res))

print(cost)
