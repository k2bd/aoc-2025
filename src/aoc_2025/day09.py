"""
External Research Done

To solve this problem, I did some external research.
I looked up algorithms to determine if a point is inside of a polygon, found
the ray-casting algorithm on Wikipedia, and adapted that for this solution.
"""

from aoc_2025.rs.day09 import FactoryFloor, largest_carpet_area


def day09_p1(puzzle_input: str) -> int:
    return largest_carpet_area(puzzle_input)


def day09_p2(puzzle_input: str) -> int:
    return FactoryFloor(puzzle_input).largest_carpet_area()
