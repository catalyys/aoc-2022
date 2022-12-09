def part1(lines):
    score = 0

    for line in lines:
        sline = line.strip()
        if sline == "A X":
            score += 3 + 1
        elif sline == "A Y":
            score += 6 + 2
        elif sline == "A Z":
            score += 0 + 3
        
        elif sline == "B X":
            score += 0 + 1
        elif sline == "B Y":
            score += 3 + 2
        elif sline == "B Z":
            score += 6 + 3
        
        elif sline == "C X":
            score += 6 + 1
        elif sline == "C Y":
            score += 0 + 2
        elif sline == "C Z":
            score += 3 + 3

    return score



def part2(lines):
    score = 0

    for line in lines:
        sline = line.strip()
        if sline == "A X":
            score += 0 + 3
        elif sline == "A Y":
            score += 3 + 1
        elif sline == "A Z":
            score += 6 + 2
        
        elif sline == "B X":
            score += 0 + 1
        elif sline == "B Y":
            score += 3 + 2
        elif sline == "B Z":
            score += 6 + 3
        
        elif sline == "C X":
            score += 0 + 2
        elif sline == "C Y":
            score += 3 + 3
        elif sline == "C Z":
            score += 6 + 1
    
    return score


if __name__ == "__main__":
    file1 = open('input', 'r')
    lines = file1.readlines()

    print(f"part 1: {part1(lines)}")
    print(f"part 2: {part2(lines)}")
