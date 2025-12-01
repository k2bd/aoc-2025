import pytest
from pytest_subtests import SubTests

from aoc_2025.core.day1 import Safe


@pytest.mark.parametrize(
    "initial, instruction, expected_pos, expected_past_zero",
    [
        (15, "L10", 5, 0),
        (5, "L10", 95, 1),
        (95, "R5", 0, 1),
        (5, "L5", 0, 1),
        (95, "R10", 5, 1),
    ],
)
def test_safe(
    initial: int,
    instruction: str,
    expected_pos: int,
    expected_past_zero: int,
    subtests: SubTests,
):
    """
    Similar test that's done in rust just to demonstrate Python interop
    """
    safe = Safe(positions=100, initial=initial)
    past_zero = safe.resolve_instruction(instruction)

    with subtests.test("Dial position"):
        assert safe.dial == expected_pos

    with subtests.test("Zero-passes"):
        assert past_zero == expected_past_zero
