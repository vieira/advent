package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"unicode"
)

func mod(a, b int) int {
	return (a%b + b) % b
}

type Position struct {
	x int
	y int
}

type Edge struct {
	a int
	b int
}

const size = 50

var tedges = map[Edge]func(*Position) (*Position, int){
	Edge{0, 50}: func(p *Position) (*Position, int) {
		return &Position{0, 2*size + p.x}, 0
	},
	Edge{0, 100}: func(p *Position) (*Position, int) {
		return &Position{p.x - 2*size, 4*size - 1}, 3
	},
	Edge{100, 0}: func(p *Position) (*Position, int) {
		return &Position{size, size + p.x}, 0
	},
}

var redges = map[Edge]func(*Position) (*Position, int){
	Edge{149, 0}: func(p *Position) (*Position, int) {
		return &Position{2*size - 1, 3*size - 1 - p.y}, 2
	},
	Edge{99, 50}: func(p *Position) (*Position, int) {
		return &Position{size + p.y, size - 1}, 3
	},
	Edge{99, 100}: func(p *Position) (*Position, int) {
		return &Position{3*size - 1, 3*size - p.y - 1}, 2
	},
	Edge{49, 150}: func(p *Position) (*Position, int) {
		return &Position{p.y - 2*size, 3*size - 1}, 3
	},
}

var bedges = map[Edge]func(*Position) (*Position, int){
	Edge{49, 100}: func(p *Position) (*Position, int) {
		return &Position{2*size - 1, p.x - size}, 2
	},
	Edge{149, 50}: func(p *Position) (*Position, int) {
		return &Position{size - 1, 2*size + p.x}, 2
	},
	Edge{199, 0}: func(p *Position) (*Position, int) {
		return &Position{2*size + p.x, 0}, 1
	},
}

var ledges = map[Edge]func(*Position) (*Position, int){
	Edge{50, 0}: func(p *Position) (*Position, int) {
		return &Position{0, 3*size - p.y - 1}, 0
	},
	Edge{50, 50}: func(p *Position) (*Position, int) {
		return &Position{p.y - size, 2 * size}, 1
	},
	Edge{0, 100}: func(p *Position) (*Position, int) {
		return &Position{size, 3*size - p.y - 1}, 0
	},
	Edge{0, 150}: func(p *Position) (*Position, int) {
		return &Position{p.y - 2*size, 0}, 1
	},
}

func wrapTop(field map[Position]rune, p *Position) (*Position, int) {
	var direction int
	fp := &Position{-1, -1}

	for tedge, f := range tedges {
		if p.y <= tedge.a && p.x >= tedge.b && p.x < tedge.b+size {
			fp, direction = f(p)
		}
	}

	if v, found := field[*fp]; !found || v == '#' {
		return p, 3
	}

	return fp, direction
}

func wrapRight(field map[Position]rune, p *Position) (*Position, int) {
	var direction int
	fp := &Position{-1, -1}

	for redge, f := range redges {
		if p.x >= redge.a && p.y >= redge.b && p.y < redge.b+size {
			fp, direction = f(p)
		}
	}

	if v, found := field[*fp]; !found || v == '#' {
		return p, 0
	}

	return fp, direction
}

func wrapBottom(field map[Position]rune, p *Position) (*Position, int) {
	var direction int
	fp := &Position{-1, -1}

	for bedge, f := range bedges {
		if p.y >= bedge.a && p.x >= bedge.b && p.x < bedge.b+size {
			fp, direction = f(p)
		}
	}

	if v, found := field[*fp]; !found || v == '#' {
		return p, 1
	}

	return fp, direction
}

func wrapLeft(field map[Position]rune, p *Position) (*Position, int) {
	var direction int
	fp := &Position{-1, -1}

	for ledge, f := range ledges {
		if p.x <= ledge.a && p.y >= ledge.b && p.y < ledge.b+size {
			fp, direction = f(p)
		}
	}

	if v, found := field[*fp]; !found || v == '#' {
		return p, 2
	}

	return fp, direction
}

type Ship struct {
	position  *Position
	direction int
}

func (s *Ship) Travel(field map[Position]rune, moves []int, turns []rune) {
	for i, move := range moves {
		for j := 0; j < move; j++ {
			switch s.direction {
			case 0: // right
				right := Position{s.position.x + 1, s.position.y}
				if v, found := field[right]; found {
					if v == '.' {
						s.position = &right
					}
					break
				}
				s.position, s.direction = wrapRight(field, s.position)
			case 1: // bottom
				bottom := Position{s.position.x, s.position.y + 1}
				if v, found := field[bottom]; found {
					if v == '.' {
						s.position = &bottom
					}
					break
				}
				s.position, s.direction = wrapBottom(field, s.position)
			case 2: // left
				left := Position{s.position.x - 1, s.position.y}
				if v, found := field[left]; found {
					if v == '.' {
						s.position = &left
					}
					break
				}
				s.position, s.direction = wrapLeft(field, s.position)
			case 3: // up
				up := Position{s.position.x, s.position.y - 1}
				if v, found := field[up]; found {
					if v == '.' {
						s.position = &up
					}
					break
				}
				s.position, s.direction = wrapTop(field, s.position)
			}
		}

		r := 1
		if i >= len(turns) {
			continue
		}
		if turns[i] == 'L' {
			r = -1
		}
		s.direction = mod(s.direction+r, 4)
	}
}

func main() {
	var start *Position
	scanner := bufio.NewScanner(os.Stdin)
	field := map[Position]rune{}
	moves := []int{}
	turns := []rune{}
	y := 0

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			break
		}

		for x, c := range line {
			if c == ' ' {
				if y == 0 {
					start = &Position{x + 1, y}
				}
				continue
			}
			field[Position{x, y}] = c
		}
		y += 1
	}

	for scanner.Scan() {
		line := scanner.Text()
		number := []rune{}
		for _, c := range line {
			if unicode.IsNumber(c) {
				number = append(number, c)
			} else {
				n, _ := strconv.Atoi(string(number))
				moves = append(moves, n)
				turns = append(turns, c)
				number = []rune{}
			}
		}

		if len(number) > 0 {
			n, _ := strconv.Atoi(string(number))
			moves = append(moves, n)
		}
	}

	s := &Ship{start, 0}
	s.Travel(field, moves, turns)
	fmt.Println(1000*(s.position.y+1) + 4*(s.position.x+1) + s.direction)
}
