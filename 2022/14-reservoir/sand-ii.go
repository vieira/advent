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

func (p *Position) Compare(q *Position) int {
	if p.x != q.x {
		return p.x - q.x
	}
	if p.y != q.y {
		return p.y - q.y
	}
	return 0
}

func (p *Position) Next(pathmap map[Position]rune) *Position {
	down := Position{p.x, p.y + 1}
	if _, found := pathmap[down]; !found {
		return &down
	}

	left := Position{p.x - 1, p.y + 1}
	if _, found := pathmap[left]; !found {
		return &left
	}

	right := Position{p.x + 1, p.y + 1}
	if _, found := pathmap[right]; !found {
		return &right
	}

	return p
}

func (p *Position) Fall(pathmap map[Position]rune, max int) *Position {
	origin := &Position{500, 0}
	curr := p
	for {
		next := curr.Next(pathmap)
		if *next == *origin {
			pathmap[*next] = 'o'
			return nil
		}
		if *curr == *next {
			pathmap[*next] = 'o'
			return next
		}
		if next.y+1 >= max {
			finish := &Position{next.x, next.y}
			pathmap[*finish] = 'o'
			return finish
		}
		curr = next
	}
}

func (p *Position) Set(pathmap map[Position]rune, max int) int {
	pathmap[*p] = '#'
	if p.y > max {
		return p.y
	}
	return max
}

func sign(n int) int {
	if n < 0 {
		return -1
	}
	return +1
}

func main() {
	var previous *Position = nil
	scanner := bufio.NewScanner(os.Stdin)
	pathmap := map[Position]rune{}
	count := 0
	max := 0

	for scanner.Scan() {
		positions := strings.Split(scanner.Text(), " -> ")
		for i, p := range positions {
			coords := strings.Split(p, ",")
			x, _ := strconv.Atoi(coords[0])
			y, _ := strconv.Atoi(coords[1])

			if i < 1 {
				previous = &Position{x, y}
				max = previous.Set(pathmap, max)
				continue
			}

			stepx := sign(previous.x - x)
			stepy := sign(previous.y - y)

			for xx := x; xx != previous.x; xx += stepx {
				(&Position{xx, y}).Set(pathmap, max)
			}

			for yy := y; yy != previous.y; yy += stepy {
				max = (&Position{x, yy}).Set(pathmap, max)
			}

			previous = &Position{x, y}
		}
	}

	for {
		count += 1
		if pos := (&Position{500, 0}).Fall(pathmap, max+2); pos == nil {
			break
		}
	}

	fmt.Println(count)
}
