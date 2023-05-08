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

func max(x, y int) int {
	if x >= y {
		return x
	}
	return y
}

func min(x, y int) int {
	return -max(-x, -y)
}

func remove(r []*Range, i int) []*Range {
	r[i] = r[len(r)-1]
	return r[:len(r)-1]
}

type Position struct {
	x int
	y int
}

type Range struct {
	start int
	end   int
}

func (r *Range) Length() int {
	return abs(r.end-r.start) + 1
}

func (r *Range) Merge(s *Range) *Range {
	if r.start <= s.start && r.end >= s.end {
		return r
	}

	if s.start <= r.start && s.end >= r.end {
		return s
	}

	if r.start >= s.start && r.start <= s.end {
		return &Range{s.start, max(s.end, r.end)}
	}

	if r.end >= s.start && r.end <= s.end {
		return &Range{min(s.start, r.start), s.end}
	}
	return nil
}

func union(rs []*Range) []*Range {
	for i := 0; i < len(rs); i++ {
		for j := i + 1; j < len(rs); j++ {
			merged := rs[i].Merge(rs[j])
			if merged != nil {
				rs[i] = merged
				rs = remove(rs, j)
				i = -1
				break
			}
		}
	}
	return rs
}

func subtract(rs []*Range, i int) []*Range {
	rgs := []*Range{}

	for _, s := range rs {
		if i >= s.start && i <= s.end {
			rgs = append(rgs, &Range{s.start, i - 1})
			rgs = append(rgs, &Range{i + 1, s.end})
		} else {
			rgs = append(rgs, s)
		}
	}
	return rgs
}

func add(ranges []*Range, p *Position, radius, y int) []*Range {
	d := radius - abs(p.y-y)
	x1 := p.x - d
	x2 := p.x + d
	return union(append(ranges, &Range{x1, x2}))
}

func main() {
	y := 2000000
	scanner := bufio.NewScanner(os.Stdin)
	beacons := []int{}
	ranges := []*Range{}

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
		if y >= p.y-r && y <= p.y+r {
			ranges = add(ranges, p, r, y)
		}

		if y == by {
			beacons = append(beacons, bx)
		}
	}

	for _, b := range beacons {
		ranges = subtract(ranges, b)
	}

	sum := 0
	for _, r := range ranges {
		sum += r.Length()
	}
	fmt.Println(sum)
}
