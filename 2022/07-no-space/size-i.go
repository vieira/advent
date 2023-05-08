package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"strconv"
)

type Stack []string

func (s *Stack) Push(str string) {
	*s = append(*s, str)
}

func (s *Stack) Pop() (string, bool) {
	if len(*s) > 0 {
		head := (*s)[len(*s)-1]
		*s = (*s)[:len(*s)-1]
		return head, true
	}
	return "", false
}

func (s *Stack) Clear() {
	*s = []string{}
}

func (s *Stack) Items() []string {
	return *s
}

func cd(cwd *Stack, path string) {
	switch path {
	case "/":
		cwd.Clear()
	case "..":
		cwd.Pop()
	default:
		cwd.Push(path)
	}
}

func ls(cwd *Stack) {
}

func cmd(cwd *Stack, args []string) {
	switch args[0] {
	case "cd":
		cd(cwd, args[1])
	case "ls":
		ls(cwd)
	default:
	}
}

func parents(cwd *Stack) []string {
	ps := []string{"/"}
	for _, p := range cwd.Items() {
		ps = append(ps, ps[len(ps)-1] + p + "/")
	}
	return ps
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	cwd := &Stack{}
	dirs := map[string]int{}
	total := 0

	for scanner.Scan() {
		line := scanner.Text()
		switch {
		case strings.HasPrefix(line, "$"):
			cmd(cwd, strings.Split(line, " ")[1:])
		case strings.HasPrefix(line, "dir"):
		default:
			file := strings.Split(line, " ")
			size, _ := strconv.Atoi(file[0])
			for _, dir := range parents(cwd) {
				dirs[dir] += size
			}
		}
	}

	for _, size := range dirs {
		if size <= 100000 {
			total += size
		}
	}

	fmt.Println(total)
}
