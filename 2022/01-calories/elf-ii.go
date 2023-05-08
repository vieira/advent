package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func remove(s []int, i int) []int {
	s[i] = s[len(s)-1]
	return s[:len(s)-1]
}

func top(s []int, n int) []int {
	if n < 1 {
		return []int{}
	}

	max := 0
	i := 0
	for j, el := range s {
		if el > max {
			max = el
			i = j
		}
	}
	return append(top(remove(s, i), n-1), max)
}

func sum(s []int) int {
	sum := 0
	for _, v := range s {
		sum += v
	}
	return sum
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

	fmt.Printf("%d\n", sum(top(calories, 3)))
}
