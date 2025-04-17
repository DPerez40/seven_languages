import logging

# configuring logging
logging.basicConfig(
    filename="app.log",
    filemode="a",
    format="%(asctime)s - %(levelname)s - %(message)s",
    level=logging.INFO
)


try:
    x = int(input("Enter a number: "))
    y = int(input("Enter a divisor: "))
    result = x / y
    print("Result is: ", result)
    logging.info(f"Division Successful: {x} / {y} = {result}")
except ZeroDivisionError:
    print("You cannot divide by zero.")
    logging.error("Attempted division by zero.")
except ValueError:
    print("You have to enter a digit, not text.")
    logging.error("Entered a non number.")
finally:
    print("Finished division attempt.")
    logging.info("Division attempt finished.")
    

    