import tkinter as tk
from tkinter import messagebox
from day31_class import BookLibrary

lib = BookLibrary()

# Functions

def add_book():
    title = entry_title.get().strip()
    author = entry_author.get().strip()
    year = entry_year.get().strip()

    if not title or not author or not year:
        messagebox.showwarning("Missing info", "All fields must be filled out.")
        return
    
    try:
        lib.add_book(title, author, year)
        update_listbox()
        entry_title.delete(0, tk.END)
        entry_author.delete(0, tk.END)
        entry_year.delete(0, tk.END)
    except Exception as e:
        messagebox.showerror("Error", str(e))

def remove_book():
    try:
        selected = listbox.get(listbox.curselection())
        lib.remove_book(selected)
        update_listbox()
    except tk.TclError:
        messagebox.showwarning("No selection", "Please select a book to remove.")

def update_listbox():
    listbox.delete(0, tk.END)
    for book in lib.books:
        listbox.insert(tk.END, book)


# UI Setup

root = tk.Tk()
root.title("Book Library Manager")

# Entry Fields
tk.Label(root, text="Title:").grid(row=0, column=0, sticky="e")
tk.Label(root, text="Author:").grid(row=1, column=0, sticky="e")
tk.Label(root, text="Year Published:").grid(row=2, column=0, sticky="e")

entry_title = tk.Entry(root, width=40)
entry_author = tk.Entry(root, width=40)
entry_year = tk.Entry(root, width=10)

entry_title.grid(row=0, column=1, padx=5, pady=5)
entry_author.grid(row=1, column=1, padx=5, pady=5)
entry_year.grid(row=2, column=1, padx=5, pady=5)

# Buttons
tk.Button(root, text="Add Book", command=add_book).grid(row=3, column=1, sticky="w", padx=5)
tk.Button(root, text="Remove Selected", command=remove_book).grid(row=3, column=1, sticky="e", padx=5)

# BookList 
listbox = tk.Listbox(root, width=60, height=10)
listbox.grid(row=4, column=0, columnspan=2, padx=5, pady=10)

update_listbox()  # initial load

root.mainloop()

