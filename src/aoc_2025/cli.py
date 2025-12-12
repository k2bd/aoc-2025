import os
import time
from dataclasses import dataclass
from typing import Annotated, Callable, Literal, Optional

import typer
from rich.console import Console
from rich.progress import track
from rich.table import Table

from aoc_2025.day01 import day01_p1, day01_p2
from aoc_2025.day02 import day02_p1, day02_p2
from aoc_2025.day03 import day03_p1, day03_p2
from aoc_2025.day04 import day04_p1, day04_p2
from aoc_2025.day05 import day05_p1, day05_p2
from aoc_2025.day06 import day06_p1, day06_p2
from aoc_2025.day07 import day07_p1, day07_p2
from aoc_2025.day08 import day08_p1_eval, day08_p1_test, day08_p2
from aoc_2025.processor import get_processor_name

DATA_DIR = os.path.join(
    os.path.dirname(os.path.dirname(os.path.dirname(__file__))), "data"
)
TEST_DIR = os.path.join(DATA_DIR, "test")
EVAL_DIR = os.path.join(DATA_DIR, "eval")

type PuzzleFunction = Callable[[str], int | str]


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
    p1: PuzzleFunction | None
    p2: PuzzleFunction | None
    p1_test: PuzzleFunction | None = None
    p2_test: PuzzleFunction | None = None
    _input_p1: str | None = None
    _input_p2: str | None = None

    @property
    def input_p1(self):
        return val if (val := self._input_p1) is not None else f"d{self.day}"

    @property
    def input_p2(self):
        return val if (val := self._input_p2) is not None else self.input_p1

    def get_runner(self, part: Literal[1, 2], is_test: bool) -> PuzzleFunction | None:
        if part == 1:
            if is_test:
                return self.p1_test or self.p1
            return self.p1
        if part == 2:
            if is_test:
                return self.p2_test or self.p2
            return self.p2


def cli(
    day_filter: Annotated[
        Optional[list[int]],
        typer.Option(
            "--day",
            "-d",
            help=(
                "One or more specific days to run. "
                "Add flag multiple times for multiple days."
            ),
            min=1,
            max=12,
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
            "--repeats",
            "-r",
            help="Number of repeats to run for average timing",
            min=1,
        ),
    ] = 1,
    no_results: Annotated[
        bool,
        typer.Option(help="Don't report results, just timings"),
    ] = False,
) -> None:
    days = [
        Day(day=1, p1=day01_p1, p2=day01_p2),
        Day(day=2, p1=day02_p1, p2=day02_p2),
        Day(day=3, p1=day03_p1, p2=day03_p2),
        Day(day=4, p1=day04_p1, p2=day04_p2),
        Day(day=5, p1=day05_p1, p2=day05_p2),
        Day(day=6, p1=day06_p1, p2=day06_p2),
        Day(day=7, p1=day07_p1, p2=day07_p2),
        Day(day=8, p1=day08_p1_eval, p1_test=day08_p1_test, p2=day08_p2),
        Day(day=9, p1=None, p2=None),
        Day(day=10, p1=None, p2=None),
        Day(day=11, p1=None, p2=None),
        Day(day=12, p1=None, p2=None),
    ]
    if day_filter:
        days = [d for d in days if d.day in day_filter]

    title = "Advent of Code 2025"
    caption = f"Run on {get_processor_name()}."

    if repeats != 1:
        caption = f"Timings show average of {repeats} runs. " + caption

    table = Table(title=title, caption=caption)
    table.add_column("Day")
    if no_results:
        table.add_column("Part 1 (ms)")
        table.add_column("Part 2 (ms)")
    else:
        table.add_column("Part 1")
        table.add_column("Time (ms)", style="italic")
        table.add_column("Part 2")
        table.add_column("Time (ms)", style="italic")

    def run_puzzle(
        *,
        input_filename: str,
        runner: PuzzleFunction,
        day_num: int,
        part_num: int,
    ) -> tuple[str | None, str | None]:
        times = []
        puzzle_input = get_puzzle_input(input_filename, test=test)
        result = None

        run_range = range(repeats)
        if repeats > 1:
            run_range = track(
                run_range,
                description=f"Day {day_num:>2} part {part_num}",
                transient=True,
            )

        for _ in run_range:
            start = time.time_ns()
            result = runner(puzzle_input)
            end = time.time_ns()
            times.append(end - start)

        entry = str(result) if result is not None else None
        time_ms = f"{(sum(times) / len(times)) / 1e6:.7}" if times else None

        return entry, time_ms

    for day in days:
        p1_entry, p1_time_ms = (
            run_puzzle(
                input_filename=day.input_p1,
                runner=runner,
                day_num=day.day,
                part_num=1,
            )
            if (runner := day.get_runner(part=1, is_test=test)) is not None
            else (None, None)
        )
        p2_entry, p2_time_ms = (
            run_puzzle(
                input_filename=day.input_p2,
                runner=runner,
                day_num=day.day,
                part_num=2,
            )
            if (runner := day.get_runner(part=2, is_test=test)) is not None
            else (None, None)
        )

        if no_results:
            table.add_row(str(day.day), p1_time_ms, p2_time_ms)
        else:
            table.add_row(str(day.day), p1_entry, p1_time_ms, p2_entry, p2_time_ms)

    console = Console()
    console.print(table)


def main():
    typer.run(cli)
