create_book <- function(title, author, year) {
  structure(list(title = title, author = author, year = year, borrowed = FALSE), class = "Book")
}


create_library <- function() {
  structure(list(books = list()), class = "Library")
}


add_book <- function(library, book) {
  library$books <- c(library$books, list(book))
  cat(paste0("Added: ", book$title, "\n"))
  return(library)
}


remove_book <- function(library, title) {
  found <- FALSE
  
  indexes <- which(sapply(library$books, function(b) b$title == title))
  
  if (length(indexes) > 0) {
    library$books <- library$books[-indexes]
    cat(paste0("Removed: ", title, "\n"))
  } else {
    cat(paste0("Book '", title, "' not found.\n"))
  }

  return(library)
}


borrow_book <- function(library, title) {
  indexes <- which(sapply(library$books, function(b) b$title == title))
  
  if (length(indexes) > 0) {
    if (library$books[[indexes]]$borrowed) {
      cat(paste0(title, " is already borrowed.\n"))
    } else {
      library$books[[indexes]]$borrowed <- TRUE
      cat(paste0(title, " has been borrowed.\n"))
    }
  } else { 
    cat(paste0("Book not found: ", title, "\n"))
  }
    
  return(library)
}


return_book <- function(library, title) {
  indexes <- which(sapply(library$books, function(b) b$title == title))
  
  if (length(indexes) > 0) {
    if (library$books[[indexes]]$borrowed) {
      library$books[[indexes]]$borrowed <- FALSE
      cat(paste0(title, " has been returned.\n"))
    } else {
      cat(paste0(title, " was never borrowed.\n"))
    }
  } else {
    cat(paste0("Book not found to return: ", title, "\n"))
  }
  
  return(library)
}


list_books <- function(library) {
  if (length(library$books) == 0) {
    cat("Library is empty..\n")
  } else {
    cat("Books in library:\n")
    for (book in library$books) {
      status <- if (book$borrowed) "Borrowed" else "Available"
      cat(paste0("- ", book$title, " by ", book$author, ", ", book$year, " (", status, ")\n"))
    }
  }
}


# Tests

lib <- create_library()
book1 <- create_book("Dune", "Frank Herbert", 1965)
book2 <- create_book("Musashi", "Eiji Yoshikawa", 1935)

lib <- add_book(lib, book1)
lib <- add_book(lib, book2)

list_books(lib)

lib <- borrow_book(lib, "Dune")
list_books(lib)

lib <- return_book(lib, "Dune")
list_books(lib)

lib <- return_book(lib, "Dune")

lib <- remove_book(lib, "Dune")
list_books(lib)
lib <- add_book(lib, book1)



