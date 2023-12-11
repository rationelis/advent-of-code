from collections import Counter

types = {
    'high_card': 1,
    'one_pair': 2,
    'two_pairs': 3,
    'three_of_a_kind': 4,
    'full_house': 5,
    'four_of_a_kind': 6,
    'five_of_a_kind': 7,
}

dic = {
    '2': 2,
    '3': 3,
    '4': 4,
    '5': 5,
    '6' : 6,
    '7' : 7,
    '8' : 8,
    '9' : 9,
    'T' : 10,
    'J' : 1,
    'Q' : 12,
    'K' : 13,
    'A' : 14,
}

def parse_input():
    with open(0) as f:
        return [line.strip() for line in f.readlines()]
    

def get_ascii_value(input):
    return list(map(lambda x: dic[x], input))


def determine_type(hand):
    jokersCount = 0
    for values in hand:
        if values == 1:
            jokersCount += 1

    old_hand = hand
    hand = list(filter(lambda x: x != 1, hand))

    element_counts = Counter(hand)

    if jokersCount == 5:
        return types['five_of_a_kind']
    if any(count == 5 for count in element_counts.values()):
        return types['five_of_a_kind']
    if any(count == 4 for count in element_counts.values()) and jokersCount == 1:
        return types['five_of_a_kind']
    if any(count == 3 for count in element_counts.values()) and jokersCount == 2:
        return types['five_of_a_kind']
    if any(count == 2 for count in element_counts.values()) and jokersCount == 3:
        return types['five_of_a_kind']
    if any(count == 1 for count in element_counts.values()) and jokersCount == 4:
        return types['five_of_a_kind']
    
    print(element_counts)

    if jokersCount == 4:
        return types['four_of_a_kind']
    if any(count == 4 for count in element_counts.values()):
        return types['four_of_a_kind']
    if any(count == 3 for count in element_counts.values()) and jokersCount == 1:
        return types['four_of_a_kind']
    if any(count == 2 for count in element_counts.values()) and jokersCount == 2:
        return types['four_of_a_kind']
    if any(count == 1 for count in element_counts.values()) and jokersCount == 3:
        return types['four_of_a_kind']
    
    pairs_count = 0
    for count in element_counts.values():
        if count == 2:
            pairs_count += 1

    if any(count == 3 for count in element_counts.values()) and any(count == 2 for count in element_counts.values()):
        return types['full_house']
    if jokersCount == 3 and any(count == 2 for count in element_counts.values()):
        return types['full_house']
    if jokersCount == 2 and any(count == 3 for count in element_counts.values()):
        return types['full_house']
    if pairs_count == 2 and jokersCount == 1:
        return types['full_house']

    if jokersCount == 3:
        return types['three_of_a_kind']
    if any(count == 3 for count in element_counts.values()):
        return types['three_of_a_kind']
    if any(count == 2 for count in element_counts.values()) and jokersCount == 1:
        return types['three_of_a_kind']
    if any(count == 1 for count in element_counts.values()) and jokersCount == 2:
        return types['three_of_a_kind']
    
    if pairs_count == 2:
        return types['two_pairs']
    if jokersCount == 2 and pairs_count == 1:
        return types['two_pairs']
    
    if pairs_count == 1:
        return types['one_pair']
    if jokersCount == 1 and pairs_count == 0:
        return types['one_pair']

    hand = old_hand

    return 0


def solve_p1(input):
    hands = []
    for line in input:
        parts = line.split(' ')
        value = int(parts[1])
        ascii = get_ascii_value(parts[0])
        hand = (determine_type(ascii), ascii, value, parts[0])
        hands.append(hand)

    hands = sorted(hands, key=lambda x: (x[0], x[1]), reverse=True)

    total = 0
    reversed_hands = hands[::-1]

    for i in range(len(reversed_hands)):    
        print(reversed_hands[i][0], reversed_hands[i][3])
        total += reversed_hands[i][2] * (i + 1)

    return total

lines = parse_input()

print(solve_p1(lines))
