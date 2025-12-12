from aoc_2025.rs.day09 import largest_carpet_area, FactoryFloor


def day09_p1(puzzle_input: str) -> int:
    return largest_carpet_area(puzzle_input)


def day09_p2(puzzle_input: str) -> int:
    return FactoryFloor(puzzle_input).largest_carpet_area()
