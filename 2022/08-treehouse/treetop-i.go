package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func key(i int, j int, rotation bool) string {
	if rotation {
		return strconv.Itoa(j) + "," + strconv.Itoa(i)
	}
	return strconv.Itoa(i) + "," + strconv.Itoa(j)
}

func count(
	trees [][]int,
	visible map[string]bool,
	rotation bool,
) map[string]bool {
	for i, row := range trees {
		max := -1
		for j, height := range row {
			if height > max {
				visible[key(i, j, rotation)] = true
				max = height
			}
		}

		max = -1
		for j := len(row) - 1; j >= 0; j-- {
			height := row[j]
			if height > max {
				visible[key(i, j, rotation)] = true
				max = height
			}
		}
	}
	return visible
}

func rotate(trees [][]int) [][]int {
	rotation := make([][]int, len(trees[0]))
	for i, _ := range rotation {
		rotation[i] = make([]int, len(trees))
		for j, _ := range rotation[i] {
			rotation[i][j] = trees[j][i]
		}
	}
	return rotation
}

func main() {
	i := 0
	scanner := bufio.NewScanner(os.Stdin)
	trees := [][]int{}

	for scanner.Scan() {
		line := scanner.Text()
		trees = append(trees, []int{})
		for _, col := range strings.Split(line, "") {
			h, _ := strconv.Atoi(col)
			trees[i] = append(trees[i], h)
		}
		i += 1
	}

	visible := map[string]bool{}
	rotation := false
	count(trees, visible, rotation)
	rotation = true
	count(rotate(trees), visible, rotation)
	fmt.Println(len(visible))
}
