from day31_class import BookLibrary

def run_library_app():
    lib = BookLibrary()

    while True:
        print("\n Welcome your personal Book Library")
        print("1. Add Book")
        print("2. Remove Book")
        print("3. List Books")
        print("4. Exit")

        choice = input("Choose an option: ").strip()

        if choice == "1":
            title = input("Enter a book title to add: ").strip()
            author = input("Enter the author: ").strip()
            year = input("Enter the year published: ").strip()
            lib.add_book(title, author, year)
        elif choice == "2":
            title = input("Enter a book to remove from the list:").strip()
            lib.remove_book(title)
        elif choice == "3":
            lib.list_books()
        elif choice == "4":
            print("Goodbye! Your books have been shelved.")
            break
        else:
            print("Invalid option. Please enter 1, 2, 3, or 4 only.")

if __name__ == "__main__":
    run_library_app()


