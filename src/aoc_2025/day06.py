from aoc_2025.rs.day06 import Homework


def day06_p1(puzzle_input: str) -> int:
    return Homework.read_wrong(puzzle_input).grand_total()


def day06_p2(puzzle_input: str) -> int:
    return Homework.read_right(puzzle_input).grand_total()
