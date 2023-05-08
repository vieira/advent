package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func stacks(scanner *bufio.Scanner) map[int][]rune {
	ss := map[int][]rune{}

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			break
		}
		for s, j := 1, 1; j < len(line); s, j = s+1, j+4 {
			c := rune(line[j])
			if !unicode.IsLetter(c) {
				continue
			}
			ss[s] = append(ss[s], c)
		}
	}
	return ss
}

func instructions(scanner *bufio.Scanner) [][]int {
	insts := [][]int{}
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}
		inst := make([]int, 3)
		tokens := strings.Split(line, " ")
		for i, j := 0, 1; i < 3; j += 2 {
			if n, err := strconv.Atoi(tokens[j]); err == nil {
				inst[i] = n
				i++
			}
		}
		insts = append(insts, inst)
	}
	return insts
}

func move(ss map[int][]rune, amount int, from int, to int) {
	for _, c := range ss[from][:amount] {
		ss[to] = append([]rune{c}, ss[to]...)
	}
	ss[from] = ss[from][amount:]
}

func top(ss map[int][]rune) string {
	code := ""
	for i := 1; i <= len(ss); i++ {
		code += string(ss[i][:1])
	}
	return code
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	ss := stacks(scanner)
	for _, inst := range instructions(scanner) {
		amount, from, to := inst[0], inst[1], inst[2]
		move(ss, amount, from, to)
	}
	fmt.Println(top(ss))
}
