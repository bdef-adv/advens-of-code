#sum([10*i[0][1] + i[-1][1] for i in 
#[sorted([(abs(offset - line.find(d if offset == 0 else d[::-1])), k+1)
#for it in [enumerate(digits), enumerate(map(str, range(1, 10)))] for k,d in it for offset, line in [(0, _line), 
#                                                                                                (len(_line)-1, _line[::-1])] 
#                                                                                                if d[:: 1 if offset == 0 else -1] in line])
#                                                                                                for _line in inp.splitlines()]])



inp = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"""

digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

for _line in inp.splitlines():
    for it in [enumerate(digits), enumerate(map(str, range(1, 10)))]:
        for k, d in it:
            for offset, line in [(0, _line), (len(_line)-1, _line[::-1])]:
                if d[:: 1 if offset == 0 else -1] in line:
                    s = sum([10*i[0][1] + i[-1][1] for i in sorted((abs(offset - line.find(d if offset == 0 else d[::-1])), k+1))])