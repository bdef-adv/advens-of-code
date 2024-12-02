#!/usr/bin/python

from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = "_2"

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        final_score = 0
        for _line in file:
            line = _line.rstrip()
            card, data = line.split(": ")
            left, right = map(str.split, data.split(' | '))
            points = 0
            for number in right:
                if number in left:
                    if not points:
                        points = 1
                    else:
                        points *= 2
            final_score += points
            #print(f"{line}: {left}, {right} ({points}) {numbers}")

        return final_score

def get_winning_cards(cards, original_cards, rec=0):
    """ PART 2 RECURSIVE SOL
    """
    won_cards = []

    for card, left, right in cards:
        _, x = card.split()
        x = int(x) - 1
        numbers = [num for num in right if num in left]
        won_cards.append((card, numbers))
        if numbers:
            won_cards += get_winning_cards(original_cards[x+1:x + 1 + len(numbers):], original_cards, rec=rec+1)
    return won_cards


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        final_score = 0
        cards = []
        for _line in file:
            line = _line.rstrip()
            card, data = line.split(": ")
            left, right = map(str.split, data.split(' | '))
            cards.append((card, left, right))

        won_cards = get_winning_cards(cards, cards)

        return len(won_cards)


if __name__ == "__main__":
    print("--- Part One ---")
    print("Test result:")
    print(solution_part1(f"input.{FILENAME_TRUNC}.test.txt"))

    print("Result:")
    print(solution_part1(f"input.{FILENAME_TRUNC}.txt"))

    print("--- Part Two ---")
    print("Test result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.test.txt"))

    print("Result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
