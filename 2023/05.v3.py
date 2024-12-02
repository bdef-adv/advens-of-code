#!/usr/bin/python

from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        seeds = []
        convert_maps = {}
        current_map = ""
        for _line in file:
            line = _line.rstrip()
            if not seeds and line.startswith("seeds:"):
                seeds = list(map(int, line.split("seeds: ")[1].split()))
            if not line:
                current_map = ""
                continue
            if seeds:
                if line.endswith("map:"):
                    current_map = line.split()[0]
                    convert_maps[current_map] = []
                elif current_map:
                    convert_maps[current_map].append(list(map(int, line.split())))

        print(convert_maps)
        seed_data = {s: {} for s in seeds}
        conversion = {}
        for name, convert_map in convert_maps.items():
            l, r = name.split("-to-")
            conversion[l] = [r, {}]
            for i in range(0, len(convert_map)):
                dest_range_start, src_range_start, range_len = convert_map[i]

                for seed in seeds:
                    print(f"{seed}: {l} to {r} => {src_range_start} - {range_len} = {dest_range_start + (src_range_start - seed)}")
                    if src_range_start <= seed < src_range_start + range_len:
                        seed_data[seed][r] = dest_range_start + (src_range_start - seed)
                    elif not seed_data[seed] or seed_data[seed][r] == seed:
                        seed_data[seed][r] = seed

        print(seed_data)

        min_location = None
        for seed, data in seed_data.items():
            if not min_location:
                min_location = data["location"]
            else:
                min_location = min(min_location, data["location"])
        
        return min_location



def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()


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
