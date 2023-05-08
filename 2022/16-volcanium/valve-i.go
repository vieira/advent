package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

const timeout = 30

type Set struct {
	elements string
}

func NewSet(elements []string) *Set {
	sort.Strings(elements)
	return &Set{strings.Join(elements[:], ",")}
}

func (s *Set) Compare(t *Set) int {
	return strings.Compare(s.elements, t.elements)
}

type Element struct {
	Id       string
	Priority int
}

type Queue struct {
	tree     []*Element
	elements map[string]bool
}

func (q *Queue) swap(x int, y int) {
	q.tree[y], q.tree[x] = q.tree[x], q.tree[y]
}

func (q *Queue) swim() {
	i := len(q.tree) - 1

	if i < 1 {
		return
	}

	child := q.tree[i]

	for i > 0 {
		pid := (i - 1) / 2
		parent := q.tree[pid]

		if parent.Priority <= child.Priority {
			return
		}

		q.swap(i, pid)
		i = pid
	}
}

func (q *Queue) sink() {
	i := 0
	sz := len(q.tree)

	if sz <= 1 {
		return
	}

	for i < sz {
		min := i
		left := (2 * i) + 1
		right := left + 1

		for _, c := range []int{left, right} {
			if c >= sz {
				continue
			}

			child := q.tree[c]
			currt := q.tree[min]

			if child.Priority < currt.Priority {
				min = c
			}
		}

		if i != min {
			q.swap(i, min)
			i = min
			continue
		}

		break
	}
}

func (q *Queue) Length() int {
	return len(q.tree)
}

func (q *Queue) Pop() *Element {
	sz := len(q.tree)
	q.swap(0, sz-1)
	el := q.tree[sz-1]
	q.tree = q.tree[:sz-1]
	q.sink()
	delete(q.elements, el.Id)
	return el
}

func (q *Queue) Push(id string, priority int) {
	q.tree = append(q.tree, &Element{id, priority})
	q.elements[id] = true
	q.swim()
}

func (q *Queue) Contains(id string) bool {
	_, found := q.elements[id]
	return found
}

type Valve struct {
	name        string
	flow        int
	connections []string
}

func (v *Valve) Compare(w *Valve) int {
	return strings.Compare(v.name, w.name)
}

func (v *Valve) AddTo(valves []*Valve) []*Valve {
	vv := []*Valve{v}
	return append(vv, valves...)
}

func dijkstra(graph map[string]*Valve, source *Valve) map[string]int {
	dist := map[string]int{}
	dist[source.name] = 0

	q := &Queue{[]*Element{}, map[string]bool{}}
	q.Push(source.name, 0)

	for q.Length() > 0 {
		u := q.Pop()

		for _, v := range graph[u.Id].connections {
			alt := dist[u.Id] + 1
			_, found := dist[v]
			if !found || alt < dist[v] {
				dist[v] = alt
				if !q.Contains(v) {
					q.Push(v, alt)
				}
			}
		}
	}

	return dist
}

func connections(valves map[string]*Valve, openable []*Valve, source string) (
	cs map[string]int,
) {
	cs = map[string]int{}
	for next, cost := range dijkstra(valves, valves[source]) {
		for _, c := range openable {
			if c.name == next {
				cs[next] = cost
			}
		}
	}
	return cs
}

type State struct {
	valve  *Valve
	open   []*Valve
	minute int
	total  int
}

func (s *State) CanOpenValve(name string) bool {
	for _, v := range s.open {
		if v.name == name {
			return false
		}
	}
	return true
}

type Graph struct {
	vertices map[string]*Valve
	edges    map[string]map[string]int
}

func successors(state *State, graph *Graph) []*State {
	ss := []*State{}

	valves := graph.vertices
	edges := graph.edges[state.valve.name]

	for name, cost := range edges {
		minute := state.minute + cost + 1

		if minute >= timeout || !state.CanOpenValve(name) {
			continue
		}

		valve := valves[name]
		open := valve.AddTo(state.open)
		total := state.total + (timeout-minute)*valve.flow
		ss = append(ss, &State{valve, open, minute, total})
	}

	return ss
}

func search(initial *State, graph *Graph) *State {
	states := []*State{initial}
	best := initial

	for len(states) > 0 {
		curr := states[0]
		states = states[1:]

		if curr.total > best.total {
			best = curr
		}

		for _, ns := range successors(curr, graph) {
			states = append(states, ns)
		}
	}

	return best
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	valves := map[string]*Valve{}
	openable := []*Valve{}

	for scanner.Scan() {
		line := strings.Split(scanner.Text(), "; ")
		valve := strings.Split(line[0], " ")
		name := valve[1]
		rate, _ := strconv.Atoi(strings.Split(valve[4], "=")[1])

		connections := strings.Split(line[1], ", ")
		connections[0] = strings.Split(connections[0], " ")[4]

		v := &Valve{name, rate, connections}
		valves[name] = v
		if v.flow > 0 {
			openable = append(openable, v)
		}
	}

	start := "AA"
	edges := map[string]map[string]int{}
	edges[start] = connections(valves, openable, start)

	for _, v := range openable {
		edges[v.name] = connections(valves, openable, v.name)
	}

	graph := &Graph{valves, edges}

	s := search(&State{valves[start], []*Valve{}, 0, 0}, graph)
	fmt.Println(s.total)
}
