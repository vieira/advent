package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func max(s []int) int {
	max := 0
	for _, el := range s {
		if el > max {
			max = el
		}
	}
	return max
}

func main() {
	var calories []int
	scanner := bufio.NewScanner(os.Stdin)
	elf := 0

	for scanner.Scan() {
		line := scanner.Text()
		if len(line) > 1 {
			if c, err := strconv.Atoi(line); err == nil {
				elf += c
			}
		} else {
			calories = append(calories, elf)
			elf = 0
		}
	}

	fmt.Printf("%d\n", max(calories))
}
