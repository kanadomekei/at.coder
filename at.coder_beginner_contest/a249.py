a, b, c, d, e, f, x = map(int, input().split())

if((x * a * b) // (a + c) > (x * d * e) // (d + e)):
    print("Takahashi")

elif((x * a * b) // (a + c) < (x * d * e) // (d + e)):
    print("Aoki")

else:
    print("Draw")