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

        conversion = {}
        for name, convert_map in convert_maps.items():
            l, r = name.split("-to-")
            conversion[l] = [r, {}]
            for i in range(0, len(convert_map)):
                dest_range_start, src_range_start, range_len = convert_map[i]

                for i in range(range_len):
                    conversion[l][1][src_range_start+i] = dest_range_start + i

                #print(f"{i} convert_map {name}: dst={dest_range_start} src={src_range_start} len={range_len} {conversion[l]}")


        seed_data = {s: {} for s in seeds}
        for seed in seeds:
            res_soil = conversion['seed'][1][seed] if seed in conversion['seed'][1] else seed
            seed_data[seed]["soil"] = res_soil

            res_fert = conversion['soil'][1][res_soil] if res_soil in conversion['soil'][1] else res_soil
            seed_data[seed]["fertilizer"] = res_fert

            res_water = conversion['fertilizer'][1][res_fert] if res_fert in conversion['fertilizer'][1] else res_fert
            seed_data[seed]["water"] = res_water

            res_light = conversion['water'][1][res_water] if res_water in conversion['water'][1] else res_water
            seed_data[seed]["light"] = res_light

            res_temp = conversion['light'][1][res_light] if res_light in conversion['light'][1] else res_light
            seed_data[seed]["temperature"] = res_temp

            res_humidity = conversion['temperature'][1][res_temp] if res_temp in conversion['temperature'][1] else res_temp
            seed_data[seed]["humidity"] = res_temp

            res_location = conversion['humidity'][1][res_humidity] if res_humidity in conversion['humidity'][1] else res_humidity
            seed_data[seed]["location"] = res_location

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
