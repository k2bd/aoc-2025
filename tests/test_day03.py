from aoc_2025.days.day03 import day03_p1, day03_p2


def test_part1(test_data):
    assert day03_p1(test_data("d3")) == 357


def test_part2(test_data):
    assert day03_p2(test_data("d3")) == 3121910778619
