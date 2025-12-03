from aoc_2025.day03 import day03_p1, day03_p2

TEST_INPUT = """987654321111111
811111111111119
234234234234278
818181911112111"""


def test_part1():
    assert day03_p1(TEST_INPUT) == 357


def test_part2():
    assert day03_p2(TEST_INPUT) == 3121910778619
