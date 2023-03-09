#!/bin/python3

import math
import os
import random
import re
import sys
import unittest
from typing import List

# Complete the minimumSwaps function below.
def minimum_swaps(arr: List[int]) -> int:
    swaps: int = 0
    for i, _ in enumerate(arr):
        while i != arr[i] - 1:
            arr[arr[i] - 1], arr[i] = arr[i], arr[arr[i] - 1]
            swaps += 1
    return swaps


if __name__ == "__main__":
    fptr = open(os.environ["OUTPUT_PATH"], "w")

    n = int(input())

    arr = list(map(int, input().rstrip().split()))

    res = minimum_swaps(arr)

    fptr.write(str(res) + "\n")

    fptr.close()


# Invoke with `python3 -m unittest minimum_swaps_2.py`
class HackerrankTests(unittest.TestCase):
    """Unit test class."""

    def test_example(self):
        self.assertEqual(minimum_swaps([7, 1, 3, 2, 4, 5, 6]), 5)

    def test_case_0(self):
        self.assertEqual(minimum_swaps([4, 3, 1, 2]), 3)

    def test_case_1(self):
        self.assertEqual(minimum_swaps([2, 3, 4, 1, 5]), 3)

    def test_case_2(self):
        self.assertEqual(minimum_swaps([1, 3, 5, 2, 4, 6, 7]), 3)
