import os
from typing import Callable

import pytest

TEST_DATA_DIR = os.path.join(os.path.dirname(os.path.dirname(__file__)), "data", "test")


@pytest.fixture
def test_data() -> Callable[[str], str]:
    def load(test_file: str) -> str:
        with open(os.path.join(TEST_DATA_DIR, test_file), "r") as f:
            return f.read()

    return load
