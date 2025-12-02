from aoc_2025.rs.day01 import Safe


def day01_p1(puzzle_input: str) -> int:
    instructions = puzzle_input.splitlines()
    safe = Safe(positions=100, initial=50)
    zero_counts = 0
    for instruction in instructions:
        safe.resolve_instruction(instruction)
        if safe.dial == 0:
            zero_counts += 1
    return zero_counts


def day01_p2(puzzle_input: str) -> int:
    instructions = puzzle_input.splitlines()
    safe = Safe(positions=100, initial=50)
    zero_counts = 0
    for instruction in instructions:
        zero_counts += safe.resolve_instruction(instruction)
    return zero_counts
