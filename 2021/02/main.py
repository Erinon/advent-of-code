def read_file(path):
    with open(path, "r") as f:
        lines = []
        for l in f.readlines():
            line = l.strip().split()
            lines.append((line[0], int(line[1])))

        return lines


def final_position(commands, use_aim):
    horizontal, depth, aim = 0, 0, 0

    for d, x in commands:
        if d == "forward":
            horizontal += x
            depth += aim * x
        elif d == "down":
            aim += x
        else:
            aim -= x

    if not use_aim:
        depth = aim

    return horizontal * depth


def main():
    commands = read_file("input.txt")

    solution_1 = final_position(commands, False)
    solution_2 = final_position(commands, True)

    print(f"Part 1: {solution_1}\n"
          f"Part 2: {solution_2}")


if __name__ == '__main__':
    main()
