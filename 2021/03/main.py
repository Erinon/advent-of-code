def read_file(path):
    with open(path, "r") as f:
        lines = []
        for l in f.readlines():
            lines.append([int(i) for i in l.strip()])

        return lines


def calculate_most_common_bits(report):
    bit_counter = [0 for _ in range(len(report[0]))]

    for params in report:
        for i, p in enumerate(params):
            bit_counter[i] += 2 * p - 1

    return list(map(lambda c: 0 if c < 0 else 1, bit_counter))


def binary_to_decimals(binary):
    decimal = 0
    xor_decimal = 0

    pow = 1
    for b in reversed(binary):
        if b == 0:
            xor_decimal += pow
        else:
            decimal += pow

        pow *= 2

    return decimal, xor_decimal


def report_filtering(report, bit_map=lambda b: b):
    i = 0
    sub_report = report
    while len(sub_report) > 1:
        most_common_bits = calculate_most_common_bits(sub_report)
        sub_report = list(
            filter(lambda b: int(b[i]) == bit_map(most_common_bits[i]), sub_report))

        i += 1

    return sub_report[0]


def calculate_power_consumption(report):
    most_common_bits = calculate_most_common_bits(report)

    gamma_rate, epsilon_rate = binary_to_decimals(most_common_bits)

    return gamma_rate * epsilon_rate


def calculate_life_support_rating(report):
    ogr_binary = report_filtering(report)
    oxygen_generator_rating, _ = binary_to_decimals(ogr_binary)

    csr_binary = report_filtering(report, lambda b: 1 - b)
    co2_scrubber_rating, _ = binary_to_decimals(csr_binary)

    return oxygen_generator_rating * co2_scrubber_rating


def main():
    report = read_file("input.txt")

    power_consumption = calculate_power_consumption(report)

    life_support_rating = calculate_life_support_rating(report)

    print(f"Part 1: {power_consumption}\n"
          f"Part 2: {life_support_rating}")


if __name__ == '__main__':
    main()
