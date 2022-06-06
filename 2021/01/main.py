def read_file(path):
    with open(path, "r") as f:
        return [int(l.strip()) for l in f.readlines()]


def cnt_increases_window(nums, wsize):
    cnt = 0
    for i, n in enumerate(nums[wsize:]):
        if n > nums[i]:
            cnt += 1

    return cnt


def main():
    nums = read_file("input.txt")

    solution_1 = cnt_increases_window(nums, 1)
    solution_2 = cnt_increases_window(nums, 3)

    print(f"Part 1: {solution_1}\n"
          f"Part 2: {solution_2}")


if __name__ == '__main__':
    main()
