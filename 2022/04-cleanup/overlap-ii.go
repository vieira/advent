package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func overlaps(s []int, t []int) bool {
	inside := func(p int, a []int) bool {
		return p >= a[0] && p <= a[1]
	}

	return inside(s[0], t) || inside(s[1], t)
}

func contains(s []int, t []int) bool {
	inside := func(s []int, t []int) bool {
		return s[0] <= t[0] && s[1] >= t[1]
	}

	return inside(s, t) || inside(t, s)
}

func intersects(s []int, t []int) bool {
	return overlaps(s, t) || contains(s, t)
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

		if intersects(section(rngs[0]), section(rngs[1])) {
			total += 1
		}
	}
	fmt.Printf("%d\n", total)
}
