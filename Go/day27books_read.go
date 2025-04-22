package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("Go/day27books.txt")
	if err != nil {
		fmt.Println("Dang, couldn't do what you asked and open the file.  I know it seems easy but one wrong character and I fall apart.", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	lineNum := 1
	fmt.Println("You Book List:")
	for scanner.Scan() {
		line := scanner.Text()
		fmt.Printf("%d. %s\n", lineNum, line)
		lineNum++
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Something went wrong reading the file: ", err)
	}
}
