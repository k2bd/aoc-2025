from aoc_2025.days.day07 import day07_p1, day07_p2


def test_part1(test_data):
    assert day07_p1(test_data("d7")) == 21


def test_part2(test_data):
    assert day07_p2(test_data("d7")) == 40
