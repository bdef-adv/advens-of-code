#!/usr/bin/python
""" day 7 """
from pathlib import Path
import sys
from functools import cmp_to_key

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

SCORING = "AKQT98765432J"
RANK = {"five of a kind": 6,
        "four of a kind": 5,
        "full house": 4,
        "three of a kind": 3,
        "two pairs": 2,
        "one pair": 1,
        "high card": 0}

def compare_cards(hand1, hand2):
    """ compare hands """
    if hand1[1] < hand2[1]:
        return -1
    if hand1[1] > hand2[1]:
        return 1

    for x, ch in enumerate(hand1[0]):
        lindex, rindex = SCORING.find(ch), SCORING.find(hand2[0][x])
        if lindex > rindex:
            return -1
        if lindex < rindex:
            return 1

    return 0

def compare_counts(hand1, hand2):
    """ compare counts """
    lindex, rindex = hand1[1], hand2[1]
    if lindex < rindex:
        return 1
    if lindex > rindex:
        return -1

    lindex, rindex = SCORING.find(hand1[0]), SCORING.find(hand2[0])
    if lindex < rindex:
        return 1
    if lindex > rindex:
        return -1
    return 0


def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        hands = []
        for _line in file:
            line = _line.rstrip()
            cards, bid = line.split()

            # card: count
            count_cards = {}
            for card in cards:
                count_cards[card] = 1 if not card in count_cards else count_cards[card] + 1

            score = ""
            pairs = []
            threes = []
            for card, count in count_cards.items():
                if count == 5:
                    score = "five of a kind"
                    break
                if count == 4:
                    score = "four of a kind"
                if count == 3:
                    threes.append(card)
                if count == 2:
                    pairs.append(card)

            if len(pairs) == 1 and len(threes) == 1:
                score = "full house"
            elif len(pairs) == 2:
                score = "two pairs"
            elif len(threes) == 1:
                score = "three of a kind"
            elif len(pairs) == 1:
                score = "one pair"
            elif not score:
                score = "high card"

            hands.append((cards, RANK[score], int(bid)))
        score = 0
        hands = sorted(hands, key=cmp_to_key(compare_cards))
        for x, hand in enumerate(hands):
            score += hand[2] * (x+1)

        return score


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        hands = []
        for _line in file:
            line = _line.rstrip()
            cards, bid = line.split()

            jokers = 0

            # card: count
            count_cards = {}
            for card in cards:
                if card == 'J':
                    jokers += 1
                else:
                    count_cards[card] = 1 if not card in count_cards else count_cards[card] + 1

            count_cards_list = sorted(count_cards.items(), key=cmp_to_key(compare_counts))

            score = ""
            pairs = []
            threes = []

            if jokers == 5:
                score = "five of a kind"

            for card, count in count_cards_list:
                if count == 5:
                    score = "five of a kind"
                    break
                if count == 4:
                    if jokers == 1:
                        score = "five of a kind"
                        jokers -= 1
                    else:
                        score = "four of a kind"
                    break
                if count == 3:
                    if jokers == 2:
                        score = "five of a kind"
                        jokers -= 2
                        break
                    elif jokers == 1:
                        score = "four of a kind"
                        jokers -= 1
                        break
                    else:
                        threes.append(card)
                if count == 2:
                    if jokers == 1:
                        jokers -= 1
                        threes.append(card)
                    elif jokers == 2:
                        score = "four of a kind"
                        jokers -= 2
                        break
                    elif jokers == 3:
                        jokers -= 3
                        score = "five of a kind"
                        break
                    else:
                        pairs.append(card)
                if count == 1:
                    if jokers == 1:
                        jokers -= 1
                        pairs.append(card)
                    elif jokers == 2:
                        jokers -= 2
                        threes.append(card)
                    elif jokers == 3:
                        jokers -= 3
                        score = "four of a kind"
                    elif jokers == 4:
                        jokers -= 4
                        score = "five of a kind"

            if len(pairs) == 1 and len(threes) == 1:
                score = "full house"
            elif len(pairs) == 2:
                score = "two pairs"
            elif len(threes) == 1:
                score = "three of a kind"
            elif len(pairs) == 1:
                score = "one pair"
            elif not score:
                score = "high card"
            hands.append((cards, RANK[score], int(bid)))
        score = 0
        hands = sorted(hands, key=cmp_to_key(compare_cards))
        for x, hand in enumerate(hands):
            score += hand[2] * (x+1)

        return score


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
