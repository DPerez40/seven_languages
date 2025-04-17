import logging
import pdb

# Logger setup
logger = logging.getLogger("InventoryLogger")
logger.setLevel(logging.INFO)

file_handler = logging.FileHandler("inventory.log")
file_handler.setFormatter(logging.Formatter("%(asctime)s - %(levelname)s - %(message)s"))
logger.addHandler(file_handler)

def add_inventory():
    try:
        title = input("Enter a title of a book: ")
        qty = int(input("Enter the quantity of the book: "))

        # debugger breakpoint
        pdb.set_trace()

        if qty <= 0:
            raise ValueError("Quantity must be greater than zero.")
        
        logger.info(f"Added '{title}' (Quantity: {qty}) to inventory.")
        print(f"Successfully added: {title} x {qty}")

        # Save to persistant storage
        with open("inventory.txt", "a") as f:
            f.write(f"{title}, {qty}\n")

    except ValueError as e:
        logger.error(f"Input error: {e}")
        print("Try again with a valid input.")

# Run
add_inventory()

