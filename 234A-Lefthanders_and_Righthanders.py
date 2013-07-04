
def solve(n, h):
	o = open("output.txt", 'w')
	n1 = 1
	n2 = int(n/2) + 1
	while(n2 <= n):
		if h[n1-1] == h[n2-1] or h[n1-1] == 'L':
			o.write(str(n1) + " " + str(n2) + "\n")
		else:
			o.write(str(n2) + " " + str(n1) + "\n")
		n1 += 1
		n2 += 1
	o.close()

if __name__ == '__main__':
	i = open("input.txt")
	n = int(i.readline())
	handed = i.readline()
	solve(n, handed)
	i.close()
