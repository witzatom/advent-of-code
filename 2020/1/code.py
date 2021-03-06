# Advent of code Year 2020 Day 1 solution
# Author = witzatom
# Date = December 2020

from itertools import combinations
from functools import reduce


def get_matching(expense_report, combination_count, to_sum):
    for combo in combinations(expense_report, combination_count):
        if sum(combo) == to_sum:
            return reduce((lambda x, y: x * y), combo)


def run(input):
    expense_report = [
        int(line)
        for line in input.split("\n")
    ]

    print("Part One : " + str(get_matching(expense_report, 2, 2020)))

    print("Part Two : " + str(get_matching(expense_report, 3, 2020)))
