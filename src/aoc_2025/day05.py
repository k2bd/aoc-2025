from aoc_2025.rs.day05 import KitchenIMS


def day05_p1(puzzle_input: str) -> int:
    return len(KitchenIMS(puzzle_input).available_and_fresh())


def day05_p2(puzzle_input: str) -> int:
    return KitchenIMS(puzzle_input).fresh_ids_count()
