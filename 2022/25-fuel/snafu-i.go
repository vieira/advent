package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func stoi(s string) int {
	i := 0
	size := len(s)
	for j, b := 0, size - 1; j < size; j, b = j + 1, b - 1 {
		a := s[j]
		p := int(math.Pow(5, float64(b)))

		switch a {
		case '-':
			i += p * -1
		case '=':
			i += p * -2
		default:
			n, _ := strconv.Atoi(string(a))
			i += p * n
		}
	}
	return i
}

func _itos(i, size int) []int {
	if size < 1 {
		return []int{i}
	}

	n := int(math.Pow(5, float64(size)))
	k := int(math.Round(float64(i)/float64(n)))
	return append([]int{k}, _itos(i - n * k, size - 1)...)
}

func itos(i int) string {
	snafu := ""
	size := int(math.Ceil(math.Log(float64(abs(i))) / math.Log(5)))
	for _, k := range _itos(i, size) {
		switch k {
		case -1:
			snafu += "-"
		case -2:
			snafu += "="
		default:
			snafu += strconv.Itoa(k)
		}
	}
	return strings.TrimLeft(snafu, "0")
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	requirements := 0

	for scanner.Scan() {
		line := scanner.Text()
		requirements += stoi(line)
	}
	fmt.Println(itos(requirements))
}
