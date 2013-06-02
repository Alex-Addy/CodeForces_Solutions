from functools import reduce

if __name__ == '__main__':
    s = input()
    c = [s.count(x) for x in set(s)]
    total = reduce(lambda a,b: a + (b%2), c, 0)
    if total % 2 == 0 and total != 0:
        print("Second")
    else:
        print("First")
