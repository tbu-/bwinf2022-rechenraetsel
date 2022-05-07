import sys

"""fuzzing tool"""

__author__ = "@functionpointer"

import aufg2_bruteforcer
import subprocess
import shlex

def parseline(line: str) -> list[int]:
    begin = line.find("{")
    end = line.find("}")
    return [int(s) for s in line[begin+1:end].split(",") if len(s)>0]

def compare(aufg: tuple[int]):
    pyresults = aufg2_bruteforcer.get_results(aufg)
    seen = set()
    py_dupes = list(set([x for x in pyresults if x in seen or seen.add(x)]))
    py_uniques = list(set([x for x in seen if x not in py_dupes]))
    py_uniques.sort()
    py_dupes.sort()

    command = shlex.split(f"target/release/rechenraetsel.exe {''.join([str(s) for s in aufg])}")
    #print(command)
    result = subprocess.run(command, capture_output=True)
    s = result.stdout.decode().split("\n")

    uniques = parseline(s[0])
    duplicates = parseline(s[1])

    is_equal = py_uniques == uniques and duplicates == py_dupes

    if not is_equal:
        print(f"task is: {aufg}")
        print(f"python uniques: {py_uniques}")
        print(f"python dupes:   {py_dupes}")
        print(f"rust uniques:   {uniques}")
        print(f"rust dupes:     {duplicates}")

    return is_equal

if __name__ == "__main__":
    import random
    import sys
    while True:
        nums = [random.randint(0, 9) for i in range(random.randint(2,6))]
        if not compare(nums):
            sys.exit(1)

