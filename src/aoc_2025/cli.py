import os
import time
from dataclasses import dataclass
from typing import Annotated, Callable, Optional, Union

import typer
from rich.console import Console
from rich.table import Table

from aoc_2025.day01 import day01_p1, day01_p2
from aoc_2025.day02 import day02_p1, day02_p2
from aoc_2025.day03 import day03_p1, day03_p2
from aoc_2025.day04 import day04_p1, day04_p2

DATA_DIR = os.path.join(
    os.path.dirname(os.path.dirname(os.path.dirname(__file__))), "data"
)
TEST_DIR = os.path.join(DATA_DIR, "test")
EVAL_DIR = os.path.join(DATA_DIR, "eval")


def get_puzzle_input(filename: str, *, test: bool) -> str:
    if test:
        dir_name = TEST_DIR
    else:
        dir_name = EVAL_DIR

    target = os.path.join(dir_name, filename)
    if not os.path.exists(target):
        relpath = os.path.relpath(target, os.getcwd())
        raise RuntimeError(f"Please add puzzle data to {relpath!r}")

    with open(target, "r") as f:
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
    repeats: Annotated[
        int,
        typer.Option(
            "--repeats", "-r", help="Number of repeats to run for average timing"
        ),
    ] = 1,
) -> None:
    days = [
        Day(day=1, p1=day01_p1, p2=day01_p2),
        Day(day=2, p1=day02_p1, p2=day02_p2),
        Day(day=3, p1=day03_p1, p2=day03_p2),
        Day(day=4, p1=day04_p1, p2=day04_p2),
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

    title = "Advent of Code 2025"
    if repeats != 1:
        title += f" ({repeats} reps)"
    table = Table(title=title)
    table.add_column("Day")
    table.add_column("Part 1")
    table.add_column("Time (ms)", style="italic")
    table.add_column("Part 2")
    table.add_column("Time (ms)", style="italic")

    for day in days:
        p1_times = []
        p2_times = []

        p1_result = None
        if day.p1 is not None:
            p1_input = get_puzzle_input(day.input_p1, test=test)
            for _ in range(repeats):
                start = time.time_ns()
                p1_result = day.p1(p1_input)
                end = time.time_ns()
                p1_times.append(end - start)

        p2_result = None
        if day.p2 is not None:
            p2_input = get_puzzle_input(day.input_p2, test=test)
            for _ in range(repeats):
                start = time.time_ns()
                p2_result = day.p2(p2_input)
                end = time.time_ns()
                p2_times.append(end - start)

        p1_time_ms = f"{(sum(p1_times) / len(p1_times)) / 1e6:.5}" if p1_times else None
        p2_time_ms = f"{(sum(p2_times) / len(p2_times)) / 1e6:.5}" if p2_times else None

        p1_entry = str(p1_result) if p1_result is not None else None
        p2_entry = str(p2_result) if p2_result is not None else None

        table.add_row(str(day.day), p1_entry, p1_time_ms, p2_entry, p2_time_ms)

    console = Console()
    console.print(table)


def main():
    typer.run(cli)
