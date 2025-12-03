from aoc_2025.rs.day03 import BatteryBank


def day03_p1(puzzle_input: str) -> int:
    batteries = puzzle_input.splitlines()
    return sum(BatteryBank(b).joltage(2) for b in batteries)


def day03_p2(puzzle_input: str) -> int:
    batteries = puzzle_input.splitlines()
    return sum(BatteryBank(b).joltage(12) for b in batteries)
