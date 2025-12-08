from aoc_2025.rs.day04 import StorageRoom


def day04_p1(puzzle_input: str) -> int:
    return len(StorageRoom(puzzle_input).accessible_rolls())


def day04_p2(puzzle_input: str) -> int:
    room = StorageRoom(puzzle_input)
    result = 0
    while removed := room.remove_accessible_rolls():
        result += len(removed)
    return result
