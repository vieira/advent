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

func wrapTop(field map[Position]rune, p *Position) *Position {
	for i := p.y; ; i-- {
		if _, found := field[Position{p.x, i}]; !found {
			if field[Position{p.x, i + 1}] == '#' {
				break
			}
			return &Position{p.x, i + 1}
		}
	}
	return p
}

func wrapRight(field map[Position]rune, p *Position) *Position {
	for i := p.x; ; i++ {
		if _, found := field[Position{i, p.y}]; !found {
			if field[Position{i - 1, p.y}] == '#' {
				break
			}
			return &Position{i - 1, p.y}
		}
	}
	return p
}

func wrapBottom(field map[Position]rune, p *Position) *Position {
	for i := p.y; ; i++ {
		if _, found := field[Position{p.x, i}]; !found {
			if field[Position{p.x, i - 1}] == '#' {
				break
			}
			return &Position{p.x, i - 1}
		}
	}
	return p
}

func wrapLeft(field map[Position]rune, p *Position) *Position {
	for i := p.x; ; i-- {
		if _, found := field[Position{i, p.y}]; !found {
			if field[Position{i + 1, p.y}] == '#' {
				break
			}
			return &Position{i + 1, p.y}
		}
	}
	return p
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
				s.position = wrapLeft(field, s.position)
			case 1: // bottom
				bottom := Position{s.position.x, s.position.y + 1}
				if v, found := field[bottom]; found {
					if v == '.' {
						s.position = &bottom
					}
					break
				}
				s.position = wrapTop(field, s.position)
			case 2: // left
				left := Position{s.position.x - 1, s.position.y}
				if v, found := field[left]; found {
					if v == '.' {
						s.position = &left
					}
					break
				}
				s.position = wrapRight(field, s.position)
			case 3: // up
				up := Position{s.position.x, s.position.y - 1}
				if v, found := field[up]; found {
					if v == '.' {
						s.position = &up
					}
					break
				}
				s.position = wrapBottom(field, s.position)
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
