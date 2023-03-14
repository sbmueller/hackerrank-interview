#!/bin/python3

import os
import unittest
from typing import List, Dict


# Complete the countTriplets function below.
def count_triplets(arr: List[int], r: int) -> int:
    triplets = 0
    singles: Dict[int, int] = {}
    pairs: Dict[int, int] = {}

    for i in reversed(arr):
        if i * r in pairs:
            triplets += pairs[i * r]
        if i * r in singles:
            pairs[i] = pairs.get(i, 0) + singles[i * r]
        singles[i] = singles.get(i, 0) + 1

    return triplets


if __name__ == "__main__":
    fptr = open(os.environ["OUTPUT_PATH"], "w")

    nr = input().rstrip().split()

    n = int(nr[0])

    r = int(nr[1])

    arr = list(map(int, input().rstrip().split()))

    ans = count_triplets(arr, r)

    fptr.write(str(ans) + "\n")

    fptr.close()


class CountTripletsTest(unittest.TestCase):
    def test_sample_0(self):
        self.assertEqual(count_triplets([1, 2, 2, 4], 2), 2)

    def test_sample_1(self):
        self.assertEqual(count_triplets([1, 3, 9, 9, 27, 81], 3), 6)

    def test_sample_2(self):
        self.assertEqual(count_triplets([1, 5, 5, 25, 125], 5), 4)

    def test_own_0(self):
        self.assertEqual(count_triplets([1, 2, 4, 4, 4, 8, 16, 32], 2), 10)

    def test_own_1(self):
        self.assertEqual(count_triplets([1, 1, 1, 1], 1), 4)

    def test_user_0(self):
        self.assertEqual(count_triplets([1, 2, 1, 2, 4], 2), 3)

    def test_own_2(self):
        self.assertEqual(count_triplets([i for i in range(999999)], 5), 39999)
