import os
import time
from dataclasses import dataclass
from typing import Annotated, Callable, Optional, Union

import typer
from rich.console import Console
from rich.table import Table

from aoc_2025.day1 import day1_p1, day1_p2

DATA_DIR = os.path.join(os.path.dirname(__file__), "data")
TEST_DIR = os.path.join(DATA_DIR, "test")
EVAL_DIR = os.path.join(DATA_DIR, "eval")


def get_puzzle_input(filename: str, *, test: bool) -> str:
    if test:
        dir_name = TEST_DIR
    else:
        dir_name = EVAL_DIR

    with open(os.path.join(dir_name, filename), "r") as f:
        return f.read()


@dataclass
class Day:
    day: int
    p1: Callable[[str], Union[int, str]] | None
    p2: Callable[[str], Union[int, str]] | None
    _input_p1: str | None = None
    _input_p2: str | None = None

    @property
    def input_p1(self):
        return val if (val := self._input_p1) is not None else f"d{self.day}"

    @property
    def input_p2(self):
        return val if (val := self._input_p2) is not None else self.input_p1


def cli(
    day_filter: Annotated[
        Optional[list[str]],
        typer.Option(
            "--day",
            "-d",
            help="One or more specific days to run",
        ),
    ] = None,
    test: Annotated[
        bool,
        typer.Option(
            "--test",
            "-t",
            help="Run test inputs instead of real inputs",
        ),
    ] = False,
) -> None:
    days = [
        Day(day=1, p1=day1_p1, p2=day1_p2),
        Day(day=2, p1=None, p2=None),
        Day(day=3, p1=None, p2=None),
        Day(day=4, p1=None, p2=None),
        Day(day=5, p1=None, p2=None),
        Day(day=6, p1=None, p2=None),
        Day(day=7, p1=None, p2=None),
        Day(day=8, p1=None, p2=None),
        Day(day=9, p1=None, p2=None),
        Day(day=10, p1=None, p2=None),
        Day(day=11, p1=None, p2=None),
        Day(day=12, p1=None, p2=None),
    ]
    if day_filter:
        days = [d for d in days if d.day in [int(day_str) for day_str in day_filter]]

    table = Table(title="Advent of Code 2025")
    table.add_column("Day")
    table.add_column("Part 1")
    table.add_column("Part 2")

    for day in days:
        if day.p1 is not None:
            p1_input = get_puzzle_input(day.input_p1, test=test)
            start = time.time_ns()
            p1_result = day.p1(p1_input)
            end = time.time_ns()
            p1_time_ms = (end - start) / 1e6
        else:
            p1_result = None
            p1_time_ms = None

        if day.p2 is not None:
            p2_input = get_puzzle_input(day.input_p2, test=test)
            start = time.time_ns()
            p2_result = day.p2(p2_input)
            end = time.time_ns()
            p2_time_ms = (end - start) / 1e6
        else:
            p2_result = None
            p2_time_ms = None

        p1_entry = f"{p1_result} ({p1_time_ms:.4f}ms)" if p1_result is not None else "-"
        p2_entry = f"{p2_result} ({p2_time_ms:.4f}ms)" if p2_result is not None else "-"

        table.add_row(str(day.day), p1_entry, p2_entry)

    console = Console()
    console.print(table)


def main():
    typer.run(cli)
