def two_adjacent_digits_are_the_same(digits):
    actual_value = digits[0]
    count = 1
    adjacent_sequences = []
    for v in digits[1:]:
        if actual_value == v:
            count += 1
        else:
            adjacent_sequences.append(count)
            actual_value = v
            count = 1
    adjacent_sequences.append(count)
    return any([v == 2 for v in adjacent_sequences])

def is_sorted(digits):
    for i in range(len(digits)-1):
        if digits[i] > digits[i + 1]:
            return False
    return True

def check(n):
    digits = [int(c) for c in str(n)]
    return is_sorted(digits) and two_adjacent_digits_are_the_same(digits)

if __name__ == "__main__":
    c = 0
    for n in range(197487, 673251 + 1):
        if check(n):
            c += 1
    print(c)

