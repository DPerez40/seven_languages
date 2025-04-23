import os

class Book:
    def __init__(self, title: str, author: str, year: int):
        self.title = title
        self.author = author
        self.year = year

    def __str__(self):
        return f"{self.title} by {self.author} ({self.year})"
    
class BookLibrary:
    def __init__(self, filename="day31books.txt"):
        self.books = []
        self.filename = filename
        self.load_books()

    def add_book(self, title: str, author: str, year: int):
        if not title.strip():
            print("Book title cannot be empty. There are no books like that.")
            return
        new_book = Book(title, author, year)
        self.books.append(new_book)
        self.save_books()
        print(f"Added book: {new_book}")

    def remove_book(self, title: str):
        if title in self.books:
            self.books.remove(title)
            self.save_books()
            print(f"Removed book: {title}")
        else:
            print(f"Book '{title}' not found.")

    def list_books(self):
        if not self.books:
            print("The Library is empty.")
            return
        print("Books in library: ")
        for idx, book in enumerate(self.books, start=1):
            print(f"{idx}. {book}")

    def save_books(self):
        with open(self.filename, "w") as f:
            for book in self.books:
                f.write(f"{book.title},{book.author},{book.year}\n")

    def load_books(self):
        if os.path.exists(self.filename):
            with open (self.filename, "r") as f:
                for line in f:
                    parts = line.strip().split(",")
                    if len(parts) == 3:
                        title, author, year = parts
                        self.books.append(Book(title, author, int(year)))




if __name__ == "__main__":
    my_library = BookLibrary()
    my_library.add_book("Dune")
    my_library.add_book("Musashi")
    my_library.list_books()
    my_library.remove_book("Dune")
    my_library.list_books()
    my_library.remove_book("The Hobbit")
