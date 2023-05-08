package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Position struct {
	x int
	y int
	z int
}

func (p *Position) Adjacents() []*Position {
	adjs := []*Position{}
	steps := []int{-1, 1}

	for _, s := range steps {
		adjs = append(adjs, &Position{p.x + s, p.y, p.z})
		adjs = append(adjs, &Position{p.x, p.y + s, p.z})
		adjs = append(adjs, &Position{p.x, p.y, p.z + s})
	}

	return adjs
}

func (p *Position) IsOutside(min, max *Position) bool {
	if p.x >= max.x || p.x <= min.x {
		return true
	}

	if p.y >= max.y || p.y <= min.y {
		return true
	}

	if p.z >= max.z || p.z <= min.z {
		return true
	}

	return false
}

func (p *Position) HasPath(cubes map[Position]int, min, max *Position) (
	bool,
	map[Position]bool,
) {
	visited := map[Position]bool{}
	ss := []*Position{p}

	for len(ss) > 0 {
		s := ss[len(ss)-1]
		ss = ss[:len(ss)-1]
		visited[*s] = true

		if s.IsOutside(min, max) {
			return true, visited
		}

		for _, a := range s.Adjacents() {
			if _, v := visited[*a]; v {
				continue
			}

			if _, c := cubes[*a]; c {
				continue
			}

			ss = append(ss, a)
		}
	}

	return false, visited
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	positions := map[Position]int{}
	max := &Position{math.MinInt, math.MinInt, math.MinInt}
	min := &Position{math.MaxInt, math.MaxInt, math.MaxInt}

	for scanner.Scan() {
		coords := strings.Split(scanner.Text(), ",")
		x, _ := strconv.Atoi(coords[0])
		y, _ := strconv.Atoi(coords[1])
		z, _ := strconv.Atoi(coords[2])

		if x > max.x {
			max.x = x
		}

		if y > max.y {
			max.y = y
		}

		if z > max.z {
			max.z = z
		}

		if x < min.x {
			min.x = x
		}

		if y < min.y {
			min.y = y
		}

		if z < min.z {
			min.z = z
		}

		positions[Position{x, y, z}] = 0
	}

	deadends := map[Position]bool{}

	for p, _ := range positions {
		for _, a := range (&p).Adjacents() {
			if _, found := positions[*a]; found {
				continue
			}

			if _, found := deadends[*a]; found {
				continue
			}

			if hasPath, visited := a.HasPath(positions, min, max); !hasPath {
				for k, _ := range visited {
					deadends[k] = true
				}
				continue
			}

			positions[p] += 1
		}
	}

	sum := 0
	for _, c := range positions {
		sum += c
	}

	fmt.Println(sum)
}
