from aoc_2025.day04 import day04_p1, day04_p2


def test_part1(test_data):
    assert day04_p1(test_data("d4")) == 13


def test_part2(test_data):
    assert day04_p2(test_data("d4")) == 43
