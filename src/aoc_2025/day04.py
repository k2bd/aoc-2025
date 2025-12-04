from aoc_2025.rs.day04 import StorageRoom


def day04_p1(puzzle_input: str) -> int:
    return len(StorageRoom(puzzle_input).accessible_rolls())
