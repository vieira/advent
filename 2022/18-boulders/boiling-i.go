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

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	positions := map[Position]int{}

	for scanner.Scan() {
		coords := strings.Split(scanner.Text(), ",")
		x, _ := strconv.Atoi(coords[0])
		y, _ := strconv.Atoi(coords[1])
		z, _ := strconv.Atoi(coords[2])
		positions[Position{x, y, z}] = 0
	}

	for p, _ := range positions {
		for _, a := range (&p).Adjacents() {
			if _, found := positions[*a]; !found {
				positions[p] += 1
			}
		}
	}

	sum := 0
	for _, c := range positions {
		sum += c
	}

	fmt.Println(sum)
}
