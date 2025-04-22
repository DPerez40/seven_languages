package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	fmt.Println("Enter a book title: ")
	title, err := reader.ReadString('\n')
	if err != nil {
		log.Fatalf("Failed to read input: %v", err)
	}
	title = strings.TrimSpace(title)

	if title == "" {
		fmt.Println("No title entered.  Try again.")
		return
	}

	//Save to file in Go folder
	file, err := os.OpenFile("Go/day27books.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		log.Fatalf("Could not open file becauseeeee: %v", err)
	}
	defer file.Close()

	_, err = file.WriteString(title + "\n")
	if err != nil {
		log.Fatalf("Failed to write because: %v", err)
	}

	fmt.Println("Book added successfully.")
}
