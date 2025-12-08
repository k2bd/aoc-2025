from aoc_2025.days.day02 import day02_p1, day02_p2


def test_part1(test_data):
    assert day02_p1(test_data("d2")) == 1227775554


def test_part2(test_data):
    assert day02_p2(test_data("d2")) == 4174379265
