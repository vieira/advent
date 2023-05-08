package main

import (
	"bufio"
	"fmt"
	"os"
)

var directions = []string{"N", "S", "W", "E"}
var restrictions = map[string][]string{
	"N": []string{"N", "NE", "NW"},
	"S": []string{"S", "SE", "SW"},
	"W": []string{"W", "NW", "SW"},
	"E": []string{"E", "NE", "SE"},
}

type Position struct {
	x int
	y int
}

func (p *Position) Adjacents() map[string]*Position {
	return map[string]*Position{
		"N":  &Position{p.x, p.y - 1},
		"S":  &Position{p.x, p.y + 1},
		"W":  &Position{p.x - 1, p.y},
		"E":  &Position{p.x + 1, p.y},
		"NE": &Position{p.x + 1, p.y - 1},
		"NW": &Position{p.x - 1, p.y - 1},
		"SE": &Position{p.x + 1, p.y + 1},
		"SW": &Position{p.x - 1, p.y + 1},
	}
}

func (p *Position) CanMove(d string, field map[Position]bool) *Position {
	adj := p.Adjacents()
	for _, r := range restrictions[d] {
		if _, found := field[*adj[r]]; found {
			return nil
		}
	}

	return adj[d]
}

func (p *Position) NeedsMove(field map[Position]bool) bool {
	for _, adj := range p.Adjacents() {
		if _, found := field[*adj]; found {
			return true
		}
	}
	return false
}

func (p *Position) Propose(field map[Position]bool, r int) *Position {
	for i := r; i < r+4; i++ {
		d := directions[i%4]
		if nextp := p.CanMove(d, field); nextp != nil {
			return nextp
		}
	}

	return nil
}

func round(field map[Position]bool, r int) int {
	proposals := map[Position][]*Position{}
	moves := 0

	for p, _ := range field {
		if p.NeedsMove(field) {
			if nextp := p.Propose(field, r); nextp != nil {
				currp := &Position{p.x, p.y}
				if _, found := proposals[*nextp]; found {
					proposals[*nextp] = append(proposals[*nextp], currp)
				} else {
					proposals[*nextp] = []*Position{currp}
				}
			}
		}
	}

	for dest, sources := range proposals {
		if len(sources) > 1 {
			continue
		}

		src := sources[0]
		field[dest] = true
		delete(field, *src)
		moves += 1
	}

	return moves
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	field := map[Position]bool{}
	y := 0

	for scanner.Scan() {
		line := scanner.Text()

		for x, c := range line {
			if c != '#' {
				continue
			}
			field[Position{x, y}] = true
		}
		y += 1
	}

	r := 0
	for round(field, r) > 0 {
		r++
	}
	fmt.Println(r + 1)
}
