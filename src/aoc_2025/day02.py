from aoc_2025.rs.day02 import get_invalid_ids_p1, get_invalid_ids_p2


def _parse_range(range_raw: str) -> tuple[int, int]:
    min_str, max_str = range_raw.split("-")
    return int(min_str), int(max_str)


def day02_p1(puzzle_input: str) -> int:
    ranges = [_parse_range(range_raw) for range_raw in puzzle_input.split(",")]
    return sum(
        invalid_id
        for invalid_ids in [
            get_invalid_ids_p1(range_min, range_max) for range_min, range_max in ranges
        ]
        for invalid_id in invalid_ids
    )


def day02_p2(puzzle_input: str) -> int:
    ranges = [_parse_range(range_raw) for range_raw in puzzle_input.split(",")]
    return sum(invalid_id for invalid_id in get_invalid_ids_p2(ranges))
