from aoc_2025.rs.day08 import Network


def day08_p1(puzzle_input: str, connections: int) -> int:
    network = Network(puzzle_input)
    network.make_connections(connections)
    circuits = network.get_circuits()

    result = 1
    for c in circuits[:3]:
        result *= len(c)
    return result


def day08_p1_test(puzzle_input: str) -> int:
    return day08_p1(puzzle_input, 10)


def day08_p1_eval(puzzle_input: str) -> int:
    return day08_p1(puzzle_input, 1000)


def day08_p2(puzzle_input: str) -> int:
    network = Network(puzzle_input)

    while True:
        c1, c2 = network.make_connection()
        circuits = network.get_circuits()
        print([len(c) for c in circuits])
        if len(circuits) == 1:
            return c1.x * c2.x
