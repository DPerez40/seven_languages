class NegativeAvgError(Exception):
    pass


try:
    age = int(input("Enter your age: "))
    if age < 0:
        raise NegativeAvgError("You can't be a time traveler.")
    print("You are", age, "yearls old.")
except NegativeAvgError as e:
    print("Cusotm error:", e)
except ValueError:
    print("Please print a valid number")
    