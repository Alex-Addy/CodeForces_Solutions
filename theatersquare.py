from math import ceil

def theater(n, m, a):
    return ceil(n/a) * ceil(m/a)

def main():
    t = input().split()
    print(theater(int(t[0]), int(t[1]), int(t[2])))

if __name__ == '__main__':
    main()
