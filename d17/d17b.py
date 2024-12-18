outputs = [2, 4, 1, 1, 7, 5, 1, 5, 4, 3, 0, 3, 5, 5, 3, 0]
outputs.reverse()

print(outputs)

def find_a(index, instructions, prev_a):
    if index >= len(instructions):
        return prev_a
    target = instructions[index]
    min_a = None
    for new_a in range(prev_a * 8 if prev_a > 0 else 1, prev_a * 8 + 8):
        c = new_a // (2 ** ((new_a % 8) ^ 1))
        b = ((new_a % 8) ^ 4 ^ c) % 8
        if b != target:
            continue
        res = find_a(index + 1, instructions, new_a)
        if min_a is None or (res is not None and res < min_a):
            min_a = res
    return min_a

print(find_a(0, outputs, 0))
