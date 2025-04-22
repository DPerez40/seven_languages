package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func safeRead() {
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("Recovered from panic: ", r)
			log.Println("Recoverd from panic: ", r)
		}
	}()

	file, err := os.Open("Go/day27books.txt")
	if err != nil {
		log.Fatal("Error opening file: ", err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	lines := []string{}
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if len(lines) == 0 {
		panic("The book list is empty!")
	}

	fmt.Println("Books list: ")
	for i, line := range lines {
		fmt.Printf("%d. %s\n", i+1, line)
	}
}

func main() {
	// Logger setup
	logFile, err := os.OpenFile("Go/day27.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		fmt.Println("Couldn't open the log file.", err)
		return
	}
	defer logFile.Close()
	log.SetOutput(logFile)

	safeRead()
}
