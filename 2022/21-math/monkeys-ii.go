package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"math/big"
)

type Job struct {
	value    *big.Rat
	operator string
	op1      string
	op2      string
}

func calc(monkey string, monkeys map[string]*Job) *big.Rat {
	job := monkeys[monkey]
	var z big.Rat
	switch job.operator {
	case "+":
		return z.Add(calc(job.op1, monkeys), calc(job.op2, monkeys))
	case "-":
		return z.Sub(calc(job.op1, monkeys), calc(job.op2, monkeys))
	case "*":
		return z.Mul(calc(job.op1, monkeys), calc(job.op2, monkeys))
	case "/":
		return z.Quo(calc(job.op1, monkeys), calc(job.op2, monkeys))
	default:
		return job.value
	}
}

func converge(m1, m2, tgt string, monkeys map[string]*Job) *big.Rat {
	left, mid, right := &big.Rat{}, &big.Rat{}, &big.Rat{}

	monkeys[tgt].value = &big.Rat{}
	mid.Sub(calc(m1, monkeys), calc(m2, monkeys))
	mid.Abs(mid)

	monkeys[tgt].value = big.NewRat(-1, 1)
	left.Sub(calc(m1, monkeys), calc(m2, monkeys))
	left.Abs(left)

	monkeys[tgt].value = big.NewRat(1, 1)
	right.Sub(calc(m1, monkeys), calc(m2, monkeys))
	right.Abs(right)

	if left.Cmp(mid) < 0 {
		r := &big.Rat{}
		d := &big.Rat{}
		d.Sub(mid, left)
		r.Quo(mid, d)
		return r
	}

	if right.Cmp(mid) < 0 {
		r := &big.Rat{}
		d := &big.Rat{}
		d.Sub(mid, right)
		r.Quo(mid, d)
		return r
	}

	return &big.Rat{}
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	monkeys := map[string]*Job{}

	for scanner.Scan() {
		line := strings.Split(scanner.Text(), ": ")
		monkey, job := line[0], line[1]
		number, err := strconv.Atoi(job)

		if err == nil {
			monkeys[monkey] = &Job{big.NewRat(int64(number), 1), "", "", ""}
			continue
		}

		op := strings.Split(job, " ")
		op1, operator, op2 := op[0], op[1], op[2]
		monkeys[monkey] = &Job{&big.Rat{}, operator, op1, op2}
	}

	root := monkeys["root"]
        c, _ := converge(root.op1, root.op2, "humn", monkeys).Float64()
	fmt.Println(int(c))
}
