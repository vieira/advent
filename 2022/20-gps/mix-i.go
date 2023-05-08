package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func mod(a, b int) int {
	return (a%b + b) % b
}

func position(i, size int) int {
	return mod(i, (size - 1))
}

type Number struct {
	value    int
	position int
	next     *Number
}

type File struct {
	numbers []*Number
	start   *Number
}

func (file *File) Move(from, to int) {
	step := 1
	v := file.numbers[from]

	if from > to {
		step = -1
	}

	for i := from; i != to; i += step {
		file.numbers[i+step].position = i
		file.numbers[i] = file.numbers[i+step]
	}
	v.position = to
	file.numbers[to] = v
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	file := &File{[]*Number{}, &Number{}}
	i := 0

	for scanner.Scan() {
		n, _ := strconv.Atoi(scanner.Text())
		file.numbers = append(file.numbers, &Number{n, i, nil})
		if i > 0 {
			file.numbers[i-1].next = file.numbers[i]
		}
		i++
	}
	file.start = file.numbers[0]

	size := len(file.numbers)
	for number := file.start; number != nil; number = number.next {
		from := number.position
		to := position(from+number.value, size)
		file.Move(from, to)
	}

	zero := 0
	for i, number := range file.numbers {
		if number.value == 0 {
			zero = i
			break
		}
	}

	sum := 0
	for _, k := range []int{1000, 2000, 3000} {
		sum += file.numbers[(zero+k)%size].value
	}

	fmt.Println(sum)
}
