from aoc_2025.day05 import day05_p1, day05_p2


def test_part1(test_data):
    assert day05_p1(test_data("d5")) == 3


def test_part2(test_data):
    assert day05_p2(test_data("d5")) == 14
