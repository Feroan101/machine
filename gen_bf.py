def string_to_bf(s):
    res = ""
    current = 0
    for char in s:
        target = ord(char)
        diff = target - current
        if diff > 0:
            res += "+" * diff
        else:
            res += "-" * abs(diff)
        res += "."
        current = target
    return res

print(string_to_bf("hello i've included this because i can"))
