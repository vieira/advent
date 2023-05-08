package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type Position struct {
	x int
	y int
}

type Chamber map[Position]bool

func (c *Chamber) Right(r *Rock) bool {
	positions := (*r).Positions()
	for _, p := range positions {
		nx := p.x + 1
		if _, hits := (*c)[Position{nx, p.y}]; nx > 7 || hits {
			return false
		}
	}
	p := positions[0]
	(*r).SetPosition(&Position{p.x + 1, p.y})
	return true
}

func (c *Chamber) Left(r *Rock) bool {
	positions := (*r).Positions()
	for _, p := range positions {
		nx := p.x - 1
		if _, hits := (*c)[Position{nx, p.y}]; nx < 1 || hits {
			return false
		}
	}
	p := positions[0]
	(*r).SetPosition(&Position{p.x - 1, p.y})
	return true
}

func (c *Chamber) Down(r *Rock) bool {
	positions := (*r).Positions()
	for _, p := range positions {
		ny := p.y - 1
		if _, hits := (*c)[Position{p.x, ny}]; ny < 1 || hits {
			return false
		}
	}
	p := positions[0]
	(*r).SetPosition(&Position{p.x, p.y - 1})
	return true
}

func (c *Chamber) Rest(r *Rock) int {
	max := 0
	positions := (*r).Positions()
	for _, p := range positions {
		if p.y > max {
			max = p.y
		}
		(*c)[*p] = true
	}
	return max
}

type Rock interface {
	GetPosition() *Position
	SetPosition(*Position)
	Positions() []*Position
}

func NewRock(t string, start *Position) Rock {
	switch t {
	case "-":
		return &Minus{start}
	case "+":
		return &Plus{&Position{start.x, start.y + 1}}
	case "L":
		return &L{start}
	case "I":
		return &I{start}
	case "o":
		return &Square{start}
	default:
		panic("unknown rock")
	}
}

type Minus struct {
	Position *Position
}

func (r *Minus) GetPosition() *Position {
	return r.Position
}

func (r *Minus) SetPosition(p *Position) {
	r.Position = p
}

func (r *Minus) Positions() []*Position {
	return []*Position{
		&Position{r.Position.x, r.Position.y},
		&Position{r.Position.x + 1, r.Position.y},
		&Position{r.Position.x + 2, r.Position.y},
		&Position{r.Position.x + 3, r.Position.y},
	}
}

type Plus struct {
	Position *Position
}

func (r *Plus) GetPosition() *Position {
	return r.Position
}

func (r *Plus) SetPosition(p *Position) {
	r.Position = p
}

func (r *Plus) Positions() []*Position {
	return []*Position{
		&Position{r.Position.x, r.Position.y},
		&Position{r.Position.x + 1, r.Position.y - 1},
		&Position{r.Position.x + 2, r.Position.y},
		&Position{r.Position.x + 1, r.Position.y},
		&Position{r.Position.x + 1, r.Position.y + 1},
	}
}

type L struct {
	Position *Position
}

func (r *L) GetPosition() *Position {
	return r.Position
}

func (r *L) SetPosition(p *Position) {
	r.Position = p
}

func (r *L) Positions() []*Position {
	return []*Position{
		&Position{r.Position.x, r.Position.y},
		&Position{r.Position.x + 1, r.Position.y},
		&Position{r.Position.x + 2, r.Position.y},
		&Position{r.Position.x + 2, r.Position.y + 1},
		&Position{r.Position.x + 2, r.Position.y + 2},
	}
}

type I struct {
	Position *Position
}

func (r *I) GetPosition() *Position {
	return r.Position
}

func (r *I) SetPosition(p *Position) {
	r.Position = p
}

func (r *I) Positions() []*Position {
	return []*Position{
		&Position{r.Position.x, r.Position.y},
		&Position{r.Position.x, r.Position.y + 1},
		&Position{r.Position.x, r.Position.y + 2},
		&Position{r.Position.x, r.Position.y + 3},
	}
}

type Square struct {
	Position *Position
}

func (r *Square) GetPosition() *Position {
	return r.Position
}

func (r *Square) SetPosition(p *Position) {
	r.Position = p
}

func (r *Square) Positions() []*Position {
	return []*Position{
		&Position{r.Position.x, r.Position.y},
		&Position{r.Position.x, r.Position.y + 1},
		&Position{r.Position.x + 1, r.Position.y + 1},
		&Position{r.Position.x + 1, r.Position.y},
	}
}

func loop(streams []string, start int, f func(string) bool) int {
	max := len(streams)

	for i := start; ; i = (i + 1) % max {
		if !f(streams[i]) {
			return i + 1
		}
	}
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	streams := []string{}
	rocks := []string{"-", "+", "L", "I", "o"}
	chamber := &Chamber{}
	iters := 10000
	sid := 0
	h := 0

	for scanner.Scan() {
		streams = strings.Split(scanner.Text(), "")
	}

	loop(rocks, 0, func(rs string) bool {
		r := NewRock(rs, &Position{3, h + 4})
		sid = loop(streams, sid, func(ss string) bool {
			if ss == ">" {
				chamber.Right(&r)
			} else {
				chamber.Left(&r)
			}

			if !chamber.Down(&r) {
				max := chamber.Rest(&r)
				if max > h {
					h = max
				}
				return false
			}
			return true
		})
		iters--
		return iters > 0
	})

	fmt.Println(h)
}
