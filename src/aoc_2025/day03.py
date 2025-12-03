from aoc_2025.rs.day03 import BatteryBank


def day03_p1(puzzle_input: str) -> int:
    batteries = puzzle_input.splitlines()
    return sum(BatteryBank(b).joltage() for b in batteries)
