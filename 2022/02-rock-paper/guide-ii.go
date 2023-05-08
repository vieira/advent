package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func score(op string, goal string) int {
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

	switch goal {
	case "X":
		return hands[wins[op]]
	case "Y":
		return 3 + hands[op]
	default:
		return 6 + hands[wins[wins[op]]]
	}
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	total := 0

	for scanner.Scan() {
		line := scanner.Text()
		play := strings.Split(line, " ")
		total += score(play[0], play[1])
	}
	fmt.Printf("%d\n", total)
}
