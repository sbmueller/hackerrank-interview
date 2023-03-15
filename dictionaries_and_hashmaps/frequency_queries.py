#!/bin/python3

import os
from collections import defaultdict
from typing import List
import unittest

# Complete the freqQuery function below.
def freq_query(queries: List[List[int]]) -> List[int]:
    # Number of occurrences for each integer in array
    frequencies: defaultdict[int, int] = defaultdict(int)
    # Number of integers that occur given amount of times. Needed to access frequencies in O(1).
    occurrences: defaultdict[int, int] = defaultdict(int)
    output = []
    for query in queries:
        command = query[0]
        value = query[1]
        if command == 3:
            if occurrences[value] > 0:
                output.append(1)
            else:
                output.append(0)
        elif command == 1:
            occurrences[frequencies[value]] -= 1
            frequencies[value] += 1
            occurrences[frequencies[value]] += 1
        elif command == 2:
            occurrences[frequencies[value]] -= 1
            frequencies[value] = frequencies[value] - 1 if frequencies[value] > 1 else 0
            occurrences[frequencies[value]] += 1
    return output


if __name__ == "__main__":
    fptr = open(os.environ["OUTPUT_PATH"], "w")

    q = int(input().strip())

    queries = []

    for _ in range(q):
        queries.append(list(map(int, input().rstrip().split())))

    ans = freq_query(queries)

    fptr.write("\n".join(map(str, ans)))
    fptr.write("\n")

    fptr.close()


class FrequencyQueriesTest(unittest.TestCase):
    def test_sample_0(self):
        self.assertEqual(
            freq_query(
                [[1, 5], [1, 6], [3, 2], [1, 10], [1, 10], [1, 6], [2, 5], [3, 2]]
            ),
            [0, 1],
        )

    def test_sample_1(self):
        self.assertEqual(
            freq_query([[3, 4], [2, 1003], [1, 16], [3, 1]]),
            [0, 1],
        )

    def test_sample_2(self):
        self.assertEqual(
            freq_query(
                [
                    [1, 3],
                    [2, 3],
                    [3, 2],
                    [1, 4],
                    [1, 5],
                    [1, 5],
                    [1, 4],
                    [3, 2],
                    [2, 4],
                    [3, 2],
                ]
            ),
            [0, 1, 1],
        )
