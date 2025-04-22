package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {

	// Setup logger
	logFile, err := os.OpenFile("Go/day27.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		fmt.Println("Couldn't open log file:", err)
		return
	}
	defer logFile.Close()
	log.SetOutput(logFile)

	// Try to open books file
	file, err := os.Open("Go/day27books.txt")
	if err != nil {
		fmt.Println("Couldn't open the book list.  Does it exist?")
		log.Println("File open error: ", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	lineNum := 1
	fmt.Println("Your Book List:")
	for scanner.Scan() {
		line := scanner.Text()
		fmt.Printf("%d. %s\n", lineNum, line)
		lineNum++
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading the file.")
		log.Println("Scan error: ", err)
	}
}
