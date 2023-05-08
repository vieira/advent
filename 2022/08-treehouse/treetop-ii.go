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

func count(trees [][]int, x int, y int) (left int, right int) {
	height := trees[y][x]
	for i := x + 1; i < len(trees[y]); i++ {
		h := trees[y][i]
		right += 1
		if h >= height {
			break
		}
	}

	for j := x - 1; j >= 0; j-- {
		h := trees[y][j]
		left += 1
		if h >= height {
			break
		}
	}

	return left, right
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
	score := 0
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

	rotation := rotate(trees)

	for x, row := range trees {
		for y, _ := range row {
			left, right := count(trees, x, y)
			up, down := count(rotation, y, x)
			s := left * right * up * down
			if s >= score {
				score = s
			}
		}
	}
	fmt.Println(score)
}
