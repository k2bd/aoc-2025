import math
from aoc_2025.rs.day07 import TachyonManifold


def day07_p1(puzzle_input: str) -> int:
    manifold = TachyonManifold(puzzle_input)
    result = 0
    while (splits := manifold.propagate()) is not None:
        result += splits
    return result


def day07_p2(puzzle_input: str) -> int:
    manifold = TachyonManifold(puzzle_input)
    result = 1
    while (splits := manifold.propagate()) is not None:
        if splits > 0:
            result = result - splits + 2**splits
    return result
