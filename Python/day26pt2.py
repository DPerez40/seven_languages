import logging

# Create custom logger
logger = logging.getLogger("BookLogger")
logger.setLevel(logging.INFO)

# File Handler
file_handler = logging.FileHandler("activity.log")
file_handler.setLevel(logging.INFO)

# Console Handler
console_handler = logging.StreamHandler()
console_handler.setLevel(logging.INFO)

# Formatter
formatter = logging.Formatter("%(asctime)s - %(levelname)s - %(message)s")
file_handler.setFormatter(formatter)
console_handler.setFormatter(formatter)

# Add handlers
logger.addHandler(file_handler)
logger.addHandler(console_handler)

# Function for book input
def add_books():
    try:
        title = input("Enter a title of a book: ")
        if not title.strip():
            raise ValueError("Empty title given")
        logger.info(f"Book added: {title}")

        # Save the book
        with open("books26.txt", "a") as file:
            file.write(title + "\n")
    except ValueError as e:
        logger.error(f"Error: {e}")



# Run
add_books()

