package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

// Logger setup
func setupLogger() {
	logFile, err := os.OpenFile("Go/day27.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		fmt.Println("Couldn't open the log file:", err)
		return
	}
	log.SetOutput(logFile)
}

func addBook(scanner *bufio.Scanner) {
	fmt.Println("Enter a book: ")
	scanner.Scan()
	title := strings.TrimSpace(scanner.Text())

	if title == "" {
		fmt.Println("Nothing was entered!")
		return
	}

	file, err := os.OpenFile("Go/day27books.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		log.Println("Error opening file:", err)
		fmt.Println("There was an error saving the book.")
		return
	}
	defer file.Close()

	_, err = file.WriteString(title + "\n")
	if err != nil {
		log.Println("Error writing to file.", err)
		fmt.Println("There was an error saving the book. See day27.log for explaination.")
		return
	}

	log.Println("Book added:", title)
	fmt.Println("Successfully added book.")
}

func listBooks() {
	file, err := os.Open("Go/day27books.txt")
	if err != nil {
		log.Println("Error opening file.", err)
		fmt.Println("Error reading the book list. See day27.log for explaination.")
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	lineNum := 1
	fmt.Println("Book list:")
	for scanner.Scan() {
		fmt.Printf("%d. %s\n", lineNum, scanner.Text())
		lineNum++
	}

	if lineNum == 1 {
		fmt.Println("No books found.")
	}
}

func main() {
	setupLogger()
	scanner := bufio.NewScanner(os.Stdin)

	for {
		fmt.Println("\nBook Manager")
		fmt.Println("------------")
		fmt.Println("1. Add Book?")
		fmt.Println("2. View Book List?")
		fmt.Println("3. Exit.")
		fmt.Println("Enter choice: ")

		scanner.Scan()
		choice := strings.TrimSpace(scanner.Text())

		switch choice {
		case "1":
			addBook(scanner)
		case "2":
			listBooks()
		case "3":
			fmt.Println("C Ya!")
			return
		default:
			fmt.Println("Choice must be 1, 2, or 3.")
		}
	}
}
