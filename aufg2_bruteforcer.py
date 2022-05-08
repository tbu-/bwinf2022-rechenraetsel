#! /usr/bin/env python3

from collections import namedtuple

__author__ = "@functionpointer"
__version__ = "1.1.1"

class MyCustomInteger(namedtuple("MyCustomInteger", "val")):

    def __add__(self, rhs):
        if self[0] is None or rhs[0] is None:
            return MyCustomInteger(None)
        return MyCustomInteger(self[0] + rhs[0])

    def __sub__(self, rhs):
        if self[0] is None or rhs[0] is None:
            return MyCustomInteger(None)
        return MyCustomInteger(self[0] - rhs[0])

    def __mul__(self, scalar):
        if self[0] is None or scalar[0] is None:
            return MyCustomInteger(None)
        return MyCustomInteger(int(self[0] * scalar[0]))

    def __truediv__(self, scalar):
        if self[0] is None or scalar[0] is None:
            return MyCustomInteger(None)
        result = self[0] / scalar[0]
        if not result.is_integer():
            return MyCustomInteger(None)
        return MyCustomInteger(int(result))

    def __float__(self):
        if self[0] is None:
            return 0.5
        return float(self[0])

def get_results(aufg: tuple[int], ops: list[str] = [], printstuff=False) -> list[int]:
    if len(ops)<len(aufg)-1:
        onext = [[*ops, s] for s in ["+", "-", "*", "/"]]
        result = []
        for o in onext:
            result += get_results(aufg, o, printstuff=printstuff)
        return result
    else:
        task = [str(val) for pair in zip(["MyCustomInteger(" + str(a) + ")" for a in aufg], ops) for val in pair] + [f"MyCustomInteger({aufg[-1]})"]
        taskstring = " ".join(task)
        try:
            result = eval(taskstring)
            if result is not None:
                result = float(result)
            else:
                return []
        except ZeroDivisionError:
            return []
        if result.is_integer():
            if printstuff:
                print(f"{result} = {taskstring.replace('MyCustomInteger(','').replace(')','')}")
            return [int(result)]
        else:
            return []

if __name__ == "__main__":
    import sys
    if len(sys.argv)==1:
        print("usage: aufg2_bruteforcer.py 4 3 2")
    else:

        task = tuple([int(i) for i in sys.argv[1:]])
        print(sorted(get_results(task, printstuff=True)))