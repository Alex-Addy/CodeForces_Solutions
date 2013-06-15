# A programming coach has n students to teach. We know that n is divisible by 3. Let's assume that all students are numbered from 1 to n, inclusive.
# Before the university programming championship the coach wants to split all students into teams of three. For some pairs of students we know that they want to be on the same team. Besides, if the i-th student wants to be on the same team with the j-th one, then the j-th student wants to be on the same team with the i-th one. The coach wants the teams to show good results, so he wants the following condition to hold: if the i-th student wants to be on the same team with the j-th, then the i-th and the j-th students must be on the same team. Also, it is obvious that each student must be on exactly one team.
# Help the coach and divide the teams the way he wants.

# Input
# The first line of the input contains integers n and m (3?=?n?=?48, . Then follow m lines, each contains a pair of integers ai,?bi (1?=?ai?<?bi?=?n) — the pair ai,?bi means that students with numbers ai and bi want to be on the same team.

# It is guaranteed that n is divisible by 3. It is guaranteed that each pair ai,?bi occurs in the input at most once.

# Output
# If the required division into teams doesn't exist, print number -1. Otherwise, print lines. In each line print three integers xi, yi, zi (1?=?xi,?yi,?zi?=?n) — the i-th team.
# If there are multiple answers, you are allowed to print any of them.


def whichList(player, teams):
    for x in range(len(teams)):
        if player in teams[x]:
            return x
    return -1

def printTeam(team):
    team = sorted(team, reverse=True)
    if len(team) == 3: print("%d %d %d" % (team[0], team[1], team[2]))
    else: print(team)

def main():
    s = input().split()
    n = int(s[0])
    m = int(s[1])
    base = list(range(1, n+1))
    teams = []
    for i in range(m):
        tmp = input().split()
        p1 = int(tmp[0])
        p2 = int(tmp[1])
        if p1 in base and p2 in base:
            teams.append([p1,p2])
            base.remove(p1)
            base.remove(p2)
        elif p1 in base:
            i = whichList(p2, teams)
            if i >= 0:
                teams[i].append(p1)
                base.remove(p1)
            else:
                print("-1")
                return
        elif p2 in base:
            i = whichList(p1, teams)
            if i >= 0:
                teams[i].append(p2)
                base.remove(p2)
            else:
                print("-1")
                return
        #print(base)
        #print(teams)

    if m == 0:
        while(len(base) >= 3):
            teams.append(base[:3])
            base = base[3:]
            
    if len(base) > 0:
        for p in base[:]:
            for t in teams:
                if len(t) > 3:
                    print("-1")
                    return
                if len(t) < 3:
                    t.append(p)
                    base.remove(p)
                    break
                
    if len(base) % 3 != 0:
        print("-1")
        return
    while(len(base) >= 3):
        teams.append(base[:3])
        base = base[3:]

    for t in teams:
         if len(t) != 3:
            print("-1")
            return
    
    # finally got down here, guess everyone is teamed up properly then
    for t in teams:
        printTeam(t)

if __name__ == '__main__':
        main()
