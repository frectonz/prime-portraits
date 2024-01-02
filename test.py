from sympy import isprime

# number = int("3828332053350533505331513"[::-1])
number = 3828332053350533505331513

if isprime(number):
    print(f"{number} is prime.")
else:
    print(f"{number} is not prime.")
