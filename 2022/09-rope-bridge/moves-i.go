package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Position struct {
	x int
	y int
}

func (p Position) Compare(q Position) int {
	if p.x != q.x {
		return p.x - q.x
	}
	if p.y != q.y {
		return p.y - q.y
	}
	return 0
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func tail(tlast Position, hnext Position) Position {
	if abs(hnext.x-tlast.x) <= 1 && abs(hnext.y-tlast.y) <= 1 {
		return tlast
	}

	y := tlast.y
	if y > hnext.y {
		y -= 1
	} else if y < hnext.y {
		y += 1
	}

	x := tlast.x
	if x > hnext.x {
		x -= 1
	} else if x < hnext.x {
		x += 1
	}

	return Position{x, y}
}

func moves(
	h Position,
	t []Position,
	dir string,
	steps int,
) (Position, []Position) {
	if steps <= 0 {
		return h, t
	}

	x, y := h.x, h.y
	hnext := h
	tlast := t[len(t)-1]

	switch dir {
	case "R":
		hnext = Position{x + 1, y}
	case "L":
		hnext = Position{x - 1, y}
	case "U":
		hnext = Position{x, y + 1}
	case "D":
		hnext = Position{x, y - 1}
	default:
		hnext = h
	}

	return moves(hnext, append(t, tail(tlast, hnext)), dir, steps-1)
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	positions := map[Position]int{}
	hnext := Position{0, 0}
	tlast := hnext

	for scanner.Scan() {
		line := scanner.Text()
		instruction := strings.Split(line, " ")
		direction := instruction[0]
		steps, _ := strconv.Atoi(instruction[1])
		h, ts := moves(hnext, []Position{tlast}, direction, steps)
		hnext = h

		for _, t := range ts {
			positions[t] += 1
			tlast = t
		}
	}
	fmt.Println(len(positions))
}
