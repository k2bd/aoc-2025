from aoc_2025.rs.day07 import TachyonManifold


def day07_p1(puzzle_input: str) -> int:
    manifold = TachyonManifold(puzzle_input)
    result = 0
    while (splits := manifold.propagate(combine_beams=True)) is not None:
        result += splits
    return result


def day07_p2(puzzle_input: str) -> int:
    manifold = TachyonManifold(puzzle_input)
    result = 1
    while (splits := manifold.propagate(combine_beams=False)) is not None:
        result += splits
    return result
