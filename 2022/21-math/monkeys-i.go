package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Job struct {
	value    int
	operator string
	op1      string
	op2      string
}

func calc(monkey string, monkeys map[string]*Job) int {
	job := monkeys[monkey]
	switch job.operator {
	case "+":
		return calc(job.op1, monkeys) + calc(job.op2, monkeys)
	case "-":
		return calc(job.op1, monkeys) - calc(job.op2, monkeys)
	case "*":
		return calc(job.op1, monkeys) * calc(job.op2, monkeys)
	case "/":
		return calc(job.op1, monkeys) / calc(job.op2, monkeys)
	default:
		return job.value
	}
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	monkeys := map[string]*Job{}

	for scanner.Scan() {
		line := strings.Split(scanner.Text(), ": ")
		monkey, job := line[0], line[1]
		number, err := strconv.Atoi(job)

		if err == nil {
			monkeys[monkey] = &Job{number, "", "", ""}
			continue
		}

		op := strings.Split(job, " ")
		op1, operator, op2 := op[0], op[1], op[2]
		monkeys[monkey] = &Job{0, operator, op1, op2}
	}

	fmt.Println(calc("root", monkeys))
}
