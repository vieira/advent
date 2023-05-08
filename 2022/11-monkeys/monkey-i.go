package main

import (
	"bufio"
	"fmt"
	"go/token"
	"go/types"
	"os"
	"strconv"
	"strings"
)

type Monkey struct {
	NumItemsInspected int
	items             []int
	operation         string
	test              int
	monkey1           int
	monkey2           int
}

func (m *Monkey) Receive(item int) {
	m.items = append(m.items, item)
}

func (m *Monkey) Operation(item int) int {
	fs := token.NewFileSet()
	expression := strings.ReplaceAll(m.operation, "old", strconv.Itoa(item))
	tv, _ := types.Eval(fs, nil, token.NoPos, expression)
	result, _ := strconv.Atoi(tv.Value.String())
	return result
}

func (m *Monkey) Turn(monkeys map[int]*Monkey) {
	for _, item := range m.items {
		worry := m.Operation(item) / 3
		monkey := monkeys[m.monkey2]
		if worry%m.test == 0 {
			monkey = monkeys[m.monkey1]
		}
		m.NumItemsInspected += 1
		monkey.Receive(worry)
	}
	m.items = []int{}
}

func NewMonkey(scanner *bufio.Scanner) *Monkey {
	Split := strings.Split

	scanner.Scan()
	items := []int{}
	for _, i := range Split(Split(scanner.Text(), ": ")[1], ", ") {
		item, _ := strconv.Atoi(i)
		items = append(items, item)
	}

	scanner.Scan()
	operation := Split(scanner.Text(), "= ")[1]

	scanner.Scan()
	test, _ := strconv.Atoi(Split(scanner.Text(), "by ")[1])

	scanner.Scan()
	monkey1, _ := strconv.Atoi(Split(scanner.Text(), "monkey ")[1])

	scanner.Scan()
	monkey2, _ := strconv.Atoi(Split(scanner.Text(), "monkey ")[1])

	return &Monkey{0, items, operation, test, monkey1, monkey2}
}

func top(monkeys map[int]*Monkey, n int) []int {
	if n < 1 {
		return []int{}
	}

	max := 0
	busiest := 0
	for m, monkey := range monkeys {
		if monkey.NumItemsInspected > max {
			max = monkey.NumItemsInspected
			busiest = m
		}
	}
	delete(monkeys, busiest)
	return append(top(monkeys, n-1), max)
}

func main() {
	i := 0
	monkeys := map[int]*Monkey{}
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, "Monkey") {
			monkeys[i] = NewMonkey(scanner)
			i++
		}
	}

	for r := 1; r <= 20; r++ {
		for m := 0; m < len(monkeys); m++ {
			monkey := monkeys[m]
			monkey.Turn(monkeys)
		}
	}

	busiest := top(monkeys, 2)
	fmt.Println(busiest[0] * busiest[1])
}
