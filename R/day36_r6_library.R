library(R6)

# Book Class

Book <- R6Class("Book",
                public = list(
                  title = NULL,
                  author = NULL,
                  year = NULL,
                  borrowed = FALSE,
                  
                  initialize = function(title, author, year) {
                    self$title <- title
                    self$author <- author
                    self$year <- year
                    self$borrowed <- FALSE
                  },
                  
                  borrow = function() {
                    if (!self$borrowed) {
                      self$borrowed <- TRUE
                      cat(paste0(self$title, " has been borrowed.\n"))
                    } else {
                      cat(paste0(self$title, " is already borrowed.\n"))
                    }
                  },
                  
                  return_book = function() {
                    if (self$borrowed) {
                      self$borrowed <- FALSE
                      cat(paste0(self$title, " has been returned.\n"))
                    } else {
                      cat(paste0(self$title, " was not borrowed.\n"))
                    }
                  }
                )
)

# Library Class:

Library <- R6Class("Library",
                   public = list(
                     books = NULL,
                     
                     initialize = function() {
                       self$books <- list()
                     },
                     
                     add_book = function(book) {
                       self$books <- c(self$books, list(book))
                       cat(paste0("Added book: ", book$title, "\n"))
                     },
                     
                     list_books = function() {
                       if (length(self$books) == 0) {
                         cat("Library is empty.\n")
                       } else {
                         cat("Books in library: \n")
                         for (book in self$books) {
                           status <- if (book$borrowed) "Borrowed" else "Available"
                           cat(paste0("- ", book$title, " by ", book$author, " (", status, ") \n"))
                         }
                       }
                     },
                     
                     search_books = function(query) {
                       found <- FALSE
                       
                       for (book in self$books) {
                         if (grepl(tolower(query), tolower(book$title)) || grepl(tolower(query), tolower(book$author))) {
                           status <- if (book$borrowed) "Borrowed" else "Available"
                           cat(paste0("- ", book$title, " by ", book$author, "(", book$year, ")\n"))
                           found <- TRUE
                         }
                       }
                       
                       if (!found) {
                         cat(paste0("No books found matching '", query, "'.\n"))
                       }
                     },
                      
                      borrow_book = function(title) {
                        index <- which(sapply(self$books, function(b) b$title == title))
                        
                        if (length(index) > 0) {
                          if (!self$books[[index]]$borrowed) {
                            self$books[[index]]$borrowed <- TRUE
                            cat(paste0(title, " has been borrowed.\n"))
                          } else {
                            cat(paste0(title, " is already borrowed.\n"))
                          }
                        } else {
                          cat(paste0(title, " not found in library.\n"))
                        }
                      },
                     
                     summary = function() {
                       total_books <- length(self$books)
                       borrowed_books <- 0
                       
                       for (book in self$books) {
                         if (book$borrowed) {
                           borrowed_books <- borrowed_books + 1
                         }
                       }
                       
                       available_books <- total_books - borrowed_books
                       
                       cat("Library Summary: \n")
                       cat("Total Books: ", total_books, "\n")
                       cat("Borrowed Books: ", borrowed_books, "\n")
                       cat("Available Books: ", available_books, "\n")
                     },
                      
                      return_book = function(title) {
                        index <- which(sapply(self$books, function(b) b$title == title))
                        
                        if (length(index) > 0) {
                          if (self$books[[index]]$borrowed) {
                            self$books[[index]]$borrowed <- FALSE
                            cat(paste0(title, " has been returned.\n"))
                          } else {
                            cat(paste0(title, " was not borrowed.\n"))
                          }
                        } else {
                          cat(paste0(title, " not found in library.\n"))
                        }
                      }
                   ))




# Test objects
dune <- Book$new("Dune", "Frank Herbert", 1965)
musashi <- Book$new("Musashi", "Eiji Yoshikawa", 1935)

dune$borrow()
dune$return_book()
musashi$return_book()

my_library <- Library$new()

my_library$add_book(dune)
my_library$add_book(musashi)
my_library$search_books("Dune")
my_library$search_books("Frank")
my_library$search_books("Star Wars")
my_library$summary()
my_library$borrow_book("Dune")
my_library$summary()

