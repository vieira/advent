package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Count map[string]int

func cp(c *Count) *Count {
	o := &Count{}

	for k, v := range *c {
		(*o)[k] = v
	}

	return o
}

type Blueprint struct {
	resources map[string]*Count
	max       *Count
}

type State struct {
	resources *Count
	robots    *Count
	time      int
}

func (s *State) TimeLeft() int {
	return 32 - s.time
}

func (s *State) HasTime(t int) bool {
	return t <= s.TimeLeft()
}

func (s *State) Geode() int {
	return (*s.resources)["geode"]
}

func (s *State) PotentialGeode() int {
	g := (*s.resources)["geode"]
	r := (*s.robots)["geode"]
	t := s.TimeLeft()

	for i := 0; i < t; i++ {
		g += (r + i)
	}
	return g
}

func (s *State) NeedsMore(robot string, blueprint *Blueprint) bool {
	if robot == "geode" {
		return true
	}

	numInstances := (*s.robots)[robot]
	maxNeeded := (*blueprint.max)[robot]

	return numInstances < maxNeeded
}

func (s *State) CanBuild(robot string, blueprint *Blueprint) (int, bool) {
	time := 1
	costs := blueprint.resources[robot]

	for unit, q := range *costs {
		if (*s.resources)[unit] < q {
			if (*s.robots)[unit] <= 0 {
				return 0, false
			}
			missing := float64(q - (*s.resources)[unit])
			numBots := float64((*s.robots)[unit])
			t := int(math.Ceil(missing / numBots))

			if t+1 > time {
				time = t + 1
			}
		}
	}

	if !s.HasTime(time) {
		return 0, false
	}

	return time, true
}

func (s *State) Successors(blueprint *Blueprint) []*State {
	ss := []*State{}

	for robot, costs := range blueprint.resources {
		if !s.NeedsMore(robot, blueprint) {
			continue
		}

		if time, canBuild := s.CanBuild(robot, blueprint); canBuild {
			resources := cp(s.resources)
			robots := cp(s.robots)

			for unit, q := range *costs {
				(*resources)[unit] -= q
			}

			for bot, n := range *robots {
				(*resources)[bot] += n * time
			}

			(*robots)[robot] += 1
			ss = append(ss, &State{resources, robots, s.time + time})
		}
	}

	return ss
}

func (s *State) Search(blueprint *Blueprint) *State {
	ss := []*State{s}
	best := s

	for len(ss) > 0 {
		s := ss[len(ss)-1]
		ss = ss[:len(ss)-1]

		if s.PotentialGeode() <= best.Geode() {
			continue
		}

		if s.Geode() > best.Geode() {
			best = s
		}

		for _, ns := range s.Successors(blueprint) {
			ss = append(ss, ns)
		}
	}

	return best
}

func parse(stream *os.File) []*Blueprint {
	scanner := bufio.NewScanner(stream)
	blueprints := []*Blueprint{}

	for scanner.Scan() {
		line := scanner.Text()
		bline := strings.Split(line[:len(line)-1], ": ")
		rline := strings.Split(bline[1], ". ")
		blueprint := &Blueprint{map[string]*Count{}, &Count{}}
		max := &Count{}

		for _, r := range rline {
			cost := &Count{}
			rw := strings.Split(r, " ")
			rname := rw[1]
			csv := strings.Split(strings.Join(rw[4:], " "), ", ")

			for i, v := range csv {
				asv := strings.Split(v, " and ")
				if len(asv) > 1 {
					csv[i] = csv[len(csv)-1]
					csv[len(csv)-1] = asv[0]

					for i := 1; i < len(asv); i++ {
						csv = append(csv, asv[i])
					}
				}
			}

			for _, c := range csv {
				cs := strings.Split(c, " ")
				n, _ := strconv.Atoi(cs[0])
				u := cs[1]
				(*cost)[u] = n

				if (*max)[u] < n {
					(*max)[u] = n
				}
			}

			blueprint.resources[rname] = cost
		}

		blueprint.max = max
		blueprints = append(blueprints, blueprint)
	}

	return blueprints
}

func main() {
	blueprints := parse(os.Stdin)
	r := &Count{"ore": 0, "clay": 0, "obsidian": 0, "geode": 0}
	b := &Count{"ore": 1, "clay": 0, "obsidian": 0, "geode": 0}

	mul := 1
	for i, blueprint := range blueprints {
		if i >= 3 {
			break
		}
		g := (&State{r, b, 0}).Search(blueprint)
		mul *= g.Geode()
	}

	fmt.Println(mul)
}
