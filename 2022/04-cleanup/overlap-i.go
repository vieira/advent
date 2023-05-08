package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func overlaps(s []int, t []int) bool {
	contains := func(s []int, t []int) bool {
		return s[0] <= t[0] && s[1] >= t[1]
	}

	return contains(s, t) || contains(t, s)
}

func section(rng string) []int {
	lr := strings.Split(rng, "-")
	l, err := strconv.Atoi(lr[0])
	r, err := strconv.Atoi(lr[1])
	if err != nil {
		return []int{0, 0}
	}
	return []int{l, r}
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	total := 0

	for scanner.Scan() {
		line := scanner.Text()
		rngs := strings.Split(line, ",")

		if overlaps(section(rngs[0]), section(rngs[1])) {
			total += 1
		}
	}
	fmt.Printf("%d\n", total)
}
