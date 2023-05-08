package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func handle(instruction string) (int, int) {
	args := strings.Split(instruction, " ")
	switch args[0] {
	case "addx":
		operand, _ := strconv.Atoi(args[1])
		return operand, 2
	case "noop":
		return 0, 1
	default:
		return 0, 0
	}
}

func findX(timeline map[int]int, cycles []int) map[int]int {
	Xs := map[int]int{}

	for _, cycle := range cycles {
		for i := cycle; i > 0; i-- {
			if X, ok := timeline[i-1]; ok {
				Xs[cycle] = X
				break
			}
		}
	}

	return Xs
}

func strength(Xs map[int]int) int {
	total := 0
	for cycle, X := range Xs {
		total += cycle * X
	}
	return total
}

func main() {
	X := 1
	clock := 0
	timeline := map[int]int{}
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		instruction := scanner.Text()
		val, cycles := handle(instruction)
		X += val
		clock += cycles
		timeline[clock] = X
	}

	s := strength(findX(timeline, []int{20, 60, 100, 140, 180, 220}))
	fmt.Println(s)
}
