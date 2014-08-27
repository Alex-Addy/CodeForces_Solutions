
def main():
    #s is strength, n is the number of dragons
    s, n = list(map(int, input().split()))

    dragons = {}
    for _ in range(n):
        strength, bonus = list(map(int, input().split()))
        dragons[strength] = bonus + dragons.get(strength, 0)

    for k in sorted(dragons.keys()):
        if k >= s:
            print("NO")
            return
        else:
            s += dragons[k]

    print("YES")

if __name__ == '__main__':
    main()
