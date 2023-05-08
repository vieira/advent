package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

func contains(lst []int, v int) bool {
	for _, el := range lst {
		if el == v {
			return true
		}
	}
	return false
}

type Position struct {
	x int
	y int
}

func (p *Position) Distance(q *Position) int {
	return abs(p.x-q.x) + abs(p.y-q.y)
}

type Sensor struct {
	position *Position
	radius   int
	lines    []*Line
}

func (s *Sensor) Covers(p *Position) bool {
	return p.Distance(s.position) <= s.radius
}

type Line struct {
	m      int
	b      int
	sensor *Sensor
}

func (l *Line) Intersection(r *Line) *Position {
	if l.m == r.m {
		return nil
	}

	x := (r.b - l.b) / (l.m - r.m)
	y := l.m*x + l.b
	p := &Position{x, y}
	dl := p.Distance(l.sensor.position)
	dr := p.Distance(r.sensor.position)

	if dl <= l.sensor.radius+1 && dr <= r.sensor.radius+1 {
		return p
	}
	return nil
}

func NewLines(s *Sensor) []*Line {
	radius := s.radius + 1

	tl := &Line{1, abs(s.position.y) + radius - abs(s.position.x), s}
	tr := &Line{-1, abs(s.position.y) + radius + abs(s.position.x), s}
	br := &Line{1, abs(s.position.y) - radius - abs(s.position.x), s}
	bl := &Line{-1, abs(s.position.y) - radius + abs(s.position.x), s}

	return []*Line{tl, tr, br, bl}
}

func main() {
	max := 4_000_000
	scanner := bufio.NewScanner(os.Stdin)
	sensors := []*Sensor{}
	intersections := map[Position]int{}

	for scanner.Scan() {
		line := strings.Split(scanner.Text(), ": ")
		scoords := strings.Split(line[0][10:], ", ")
		bcoords := strings.Split(line[1][21:], ", ")
		sx, _ := strconv.Atoi(strings.Split(scoords[0], "=")[1])
		sy, _ := strconv.Atoi(strings.Split(scoords[1], "=")[1])
		bx, _ := strconv.Atoi(strings.Split(bcoords[0], "=")[1])
		by, _ := strconv.Atoi(strings.Split(bcoords[1], "=")[1])

		p := &Position{sx, sy}
		r := abs(bx-sx) + abs(by-sy)
		s := &Sensor{p, r, []*Line{}}
		s.lines = append(s.lines, NewLines(s)...)
		sensors = append(sensors, s)
	}

	for i := 0; i < len(sensors); i++ {
		for j := i + 1; j < len(sensors); j++ {
			for _, l := range sensors[i].lines {
				for _, m := range sensors[j].lines {
					if p := l.Intersection(m); p != nil {
						intersections[*p] += 1
					}
				}
			}
		}
	}

	for p, count := range intersections {
		covered := false
		if count < 4 {
			continue
		}
		for _, sensor := range sensors {
			if sensor.Covers(&p) {
				covered = true
			}
		}
		if !covered {
			fmt.Println(p.x*max + p.y)
		}
	}
}
