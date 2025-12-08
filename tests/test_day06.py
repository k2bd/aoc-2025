from aoc_2025.days.day06 import day06_p1, day06_p2


def test_part1(test_data):
    assert day06_p1(test_data("d6")) == 4277556


def test_part2(test_data):
    assert day06_p2(test_data("d6")) == 3263827
