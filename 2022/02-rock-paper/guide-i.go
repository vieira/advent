package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func score(op string, me string) int {
	hands := map[string]int{
		"A": 1,
		"B": 2,
		"C": 3,
	}
	wins := map[string]string{
		"A": "C",
		"B": "A",
		"C": "B",
	}
	score := hands[me]

	if op == me {
		return score + 3
	}

	if wins[me] == op {
		return score + 6
	}

	return score
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	total := 0
	assumptions := map[string]string{
		"X": "A",
		"Y": "B",
		"Z": "C",
	}

	for scanner.Scan() {
		line := scanner.Text()
		plays := strings.Split(line, " ")
		total += score(plays[0], assumptions[plays[1]])
	}
	fmt.Printf("%d\n", total)
}
