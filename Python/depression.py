if __name__ == '__main__':
    ti = input()
    ti = ti.split(':')
    hr = int(ti[0])
    mi = int(ti[1])
    turn_min = mi * 6
    turn_hour = (hr%12)*30 + mi*0.5
        
    print(turn_hour, turn_min)
