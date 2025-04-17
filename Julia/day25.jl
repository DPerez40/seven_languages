function safe_read(filename)
    try
        lines = readlines(filename)
        println("Reading file content:")
        for line in lines
            println("Book: ", line)
        end
    catch e
        println("Cuidado my friend, I couldn't read that because: ", e)
    end
end

# Count the number of non-empty lines in the file
function count_lines(filename)
    try
        lines = readlines(filename)
        non_empty = filter(line -> !isempty(strip(line)), lines)
        println("This file contains ", length(non_empty), " lines.")
    catch e
        println("Could not read the file: ", e)
    end
end

# Show recent books
function show_recent_books(filename)
    try
        lines = readlines(filename)
        println("\nBooks published after 1950:")

        for line in lines
            if occursin(r"\d{4}", line)
                year_match = match(r"\d{4}", line)

                if year_match !== nothing
                    year = parse(Int, year_match.match)
                    if year > 1950
                        println("Book: ", line, " (", year, ")")
                    end
                end
            end
        end
    catch e
        println("Could not filter books:", e)
    end
end


# Run
safe_read("books.txt")
count_lines("books.txt")
show_recent_books("books.txt")

safe_read("bookss.txt")
count_lines("bookss.txt")
show_recent_books("bookss.txt")
