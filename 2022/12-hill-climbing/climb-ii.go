package main

import (
	"bufio"
	"fmt"
	"os"
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

func (p *Position) Adjacents(hillmap [][]rune) []*Position {
	height := len(hillmap)
	width := len(hillmap[0])
	adjacents := []*Position{}
	moves := []int{-1, +1}

	for _, m := range moves {
		x := p.x + m
		if x < 0 || x >= width || hillmap[p.y][p.x]-1 > hillmap[p.y][x] {
			continue
		}
		adjacents = append(adjacents, &Position{x, p.y})
	}

	for _, m := range moves {
		y := p.y + m
		if y < 0 || y >= height || hillmap[p.y][p.x]-1 > hillmap[y][p.x] {
			continue
		}
		adjacents = append(adjacents, &Position{p.x, y})
	}

	return adjacents
}

func isGoal(path []*Position, hillmap [][]rune) bool {
	position := path[len(path)-1]
	return hillmap[position.y][position.x] <= 'a'
}

func successors(path []*Position, hillmap [][]rune) [][]*Position {
	position := path[len(path)-1]
	ss := [][]*Position{}

	for _, a := range position.Adjacents(hillmap) {
		s := make([]*Position, len(path))
		copy(s, path)
		s = append(s, a)
		ss = append(ss, s)
	}

	return ss
}

func search(start []*Position, hillmap [][]rune) []*Position {
	pending := [][]*Position{start}
	fastest := map[Position]int{}

	for {
		if len(pending) < 1 {
			break
		}
		path := pending[0]
		pending = pending[1:]

		if isGoal(path, hillmap) {
			return path[1:]
		}

		for _, s := range successors(path, hillmap) {
			pos := s[len(s)-1]
			steps, found := fastest[*pos]
			if !found || steps > len(s) {
				pending = append(pending, s)
				fastest[*pos] = len(s)
			}
		}
	}

	return []*Position{}
}

func main() {
	var start *Position
	scanner := bufio.NewScanner(os.Stdin)
	hillmap := [][]rune{}
	y := 0

	for scanner.Scan() {
		line := scanner.Text()
		row := []rune{}
		for x, ch := range line {
			switch ch {
			case 'S':
				row = append(row, 'a'-1)
			case 'E':
				row = append(row, 'z'+1)
				start = &Position{x, y}
			default:
				row = append(row, ch)
			}
		}
		y++
		hillmap = append(hillmap, row)
	}

	path := search([]*Position{start}, hillmap)
	fmt.Println(len(path))
}
