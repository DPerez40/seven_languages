class Book {
    title: string;
    author: string;
    year: number;

    constructor(title: string, author: string, year: number) {
        this.title = title;
        this.author = author;
        this.year = year;
    }

    summary(): string {
        return `${this.title} by ${this.author}, published in ${this.year}.`;
    }
}

// Usage

const book = new Book("Dune", "Frank Herbert", 1965);
console.log(book.summary());

class Library {
    private books: Book[] =[];

    addBook(book: Book): void {
        if (!book.title.trim()) {
            console.log("Title cannot be empty.");
            return;
        }

        if (isNaN(book.year) || book.year < 0 || book.year > new Date().getFullYear()) {
            console.log("Year must be valid. No books written BS and no books from the future.");
            return;
        }

        const exists = this.books.some(b => b.title.toLowerCase() === book.title.toLowerCase());
        if (exists) {
            console.log(`"${book.title}" is already in the library.`);
            return;
        }

        this.books.push(book);
        console.log(`Added: ${book.title}`);
    }

    findBook(title: string): Book | undefined {
        return this.books.find(b => b.title.toLowerCase() === title.toLowerCase());
    }

    removeBook(title: string): void {
        const index = this.books.findIndex(b => b.title === title);
        if (index === -1) {
            console.log(`"${title}" not found.`);
            return;
        }

        this.books.splice(index, 1);
        console.log(`Removed: ${title}`);
    }

    getBooksByAuthor(author: string): Book[] {
        return this.books.filter(b => b.author.toLowerCase() === author.toLowerCase());
    }

    listBooks(): void {
        if (this.books.length === 0) {
            console.log("Library is empty.");
            return;
        }

        console.log("Books in library:");
        for (const book of this.books) {
            console.log("- " + book.summary())
        }
    }
}

const myLibrary = new Library();

myLibrary.addBook(new Book("Dune", "Frank Herbert", 1965));
myLibrary.addBook(new Book("   ", "Nobody", 2024));
myLibrary.addBook(new Book("Musashi", "Eiji Yoshikawa", 1935));

const found = myLibrary.findBook("dune");
if (found) {
    console.log("Found book:", found.summary());
    console.log(`Published in ${found.year}, by ${found.author}`);
} else {
    console.log("Book not found.");
}


myLibrary.listBooks();

const yoshikawaBooks = myLibrary.getBooksByAuthor("Eiji Yoshikawa");
console.log("Books by Eiji Yoshikawa:");
yoshikawaBooks.forEach(b => console.log("- " + b.summary()));

