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

func findX(timeline map[int]int, cycle int) int {
	for i := cycle; i > 0; i-- {
		if X, ok := timeline[i-1]; ok {
			return X
		}
	}
	return -1
}

func printCRT(timeline map[int]int, clock int) {
	for i := 1; i <= clock; i++ {
		j := i % 40
		sprite := findX(timeline, i)
		ch := "."

		if j >= sprite && j <= sprite+2 {
			ch = "#"
		}

		fmt.Printf("%s", ch)

		if j == 0 {
			fmt.Println()
		}
	}
}

func main() {
	X := 1
	clock := 0
	timeline := map[int]int{clock: X}
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		instruction := scanner.Text()
		val, cycles := handle(instruction)
		X += val
		clock += cycles
		timeline[clock] = X
	}

	printCRT(timeline, clock)
}
