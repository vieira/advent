package main

import (
	"bufio"
	"fmt"
	"os"
)

func mod(a, b int) int {
	return (a%b + b) % b
}

type Field struct {
	positions map[Position]rune
	x         map[int][]*Position
	y         map[int][]*Position
	start     *Position
	goal      *Position
	size      *Position
}

func (f *Field) Add(p *Position, c rune) {
	f.positions[*p] = c
	f.x[p.x] = append(f.x[p.x], p)
	f.y[p.y] = append(f.y[p.y], p)
}

func (f *Field) HasBlizzard(p *Position, r int) bool {
  left, right := p.Next('<', r, f.size), p.Next('>', r, f.size)
  up, down := p.Next('^', r, f.size), p.Next('v', r, f.size)

  if  f.positions[*left] == '>' {
    return true
  }

  if f.positions[*right] == '<' {
    return true
  }

  if f.positions[*up] == 'v' {
    return true
  }

  if f.positions[*down] == '^' {
    return true
  }

  return false
}

type Position struct {
	x int
	y int
}

func (p *Position) Adjacents(field *Field) []*Position {
	adjacents := []*Position{}

	for _, dx := range []int{-1, 0, 1} {
		for _, dy := range []int{-1, 0, 1} {
			if dx != 0 && dy != 0 {
				continue
			}
			np := &Position{p.x + dx, p.y + dy}
			if *np == *field.start || *np == *field.goal {
				adjacents = append(adjacents, np)
				continue
			}

			if np.x < 1 || np.x >= field.size.x {
				continue
			}

			if np.y < 1 || np.y >= field.size.y {
				continue
			}

			adjacents = append(adjacents, np)
		}
	}

	return adjacents
}

func (p *Position) Next(d rune, r int, s *Position) *Position {
	switch d {
	case '^':
		y := 1 + mod(p.y-1-r, s.y-1)
		return &Position{p.x, y}
	case 'v':
		y := 1 + mod(p.y-1+r, s.y-1)
		return &Position{p.x, y}
	case '<':
		x := 1 + mod(p.x-1-r, s.x-1)
		return &Position{x, p.y}
	case '>':
		x := 1 + mod(p.x-1+r, s.x-1)
		return &Position{x, p.y}
	default:
		return p
	}
}

type State struct {
	p *Position
	r int
}

func (s *State) Successors(f *Field) []*State {
	ss := []*State{}
	for _, a := range s.p.Adjacents(f) {
		if !f.HasBlizzard(a, s.r) {
			ss = append(ss, &State{a, s.r + 1})
		}
	}
	return ss
}

func (s *State) IsGoal(goal *Position) bool {
	return *(s.p) == *goal
}

func search(initial *State, f *Field) int {
	q := []*State{initial}
	visited := map[Position]int{}

	for len(q) > 0 {
		s := q[0]
		q = q[1:]

		if s.IsGoal(f.goal) {
			return s.r - 1
		}

		for _, ns := range s.Successors(f) {
			if c, found := visited[*ns.p]; !found || c != ns.r {
				q = append(q, ns)
				visited[*ns.p] = ns.r
			}
		}
	}

	return -1
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	field := &Field{
		map[Position]rune{},
		map[int][]*Position{},
		map[int][]*Position{},
		nil,
		nil,
		&Position{},
	}
	y := 0

	for scanner.Scan() {
		line := scanner.Text()

		for x, c := range line {
			if field.start == nil && c == '.' {
				field.start = &Position{x, y}
				continue
			}

			if c != '#' && c != '.' {
				field.Add(&Position{x, y}, c)
				continue
			}

			if c == '.' {
				field.goal = &Position{x, y}
			}

			if c == '#' {
				field.size = &Position{x, y}
			}
		}
		y += 1
	}

	first := search(&State{field.start, 1}, field)

	field.start, field.goal = field.goal, field.start
	back := search(&State{field.start, first + 1}, field)

	field.start, field.goal = field.goal, field.start
	forth := search(&State{field.start, back + 1}, field)
	fmt.Println(forth)
}
