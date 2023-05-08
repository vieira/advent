package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type Node struct {
	children []*Node
	value    int
}

func NewParent(value int) *Node {
	return &Node{value: -1, children: []*Node{&Node{value: value}}}
}

func NewNode() *Node {
	return &Node{value: -1}
}

func (n *Node) Compare(p *Node) int {
	if n.value >= 0 && p.value >= 0 {
		return n.value - p.value
	}

	if n.value < 0 && p.value < 0 {
		nl := len(n.children)
		pl := len(p.children)

		for i := 0; i < nl && i < pl; i++ {
			nc := n.children[i]
			pc := p.children[i]

			if nc.Compare(pc) == 0 {
				continue
			}

			return nc.Compare(pc)
		}
		return nl - pl
	}

	if n.value < 0 {
		cp := NewParent(p.value)
		return n.Compare(cp)
	}

	if p.value < 0 {
		cn := NewParent(n.value)
		return cn.Compare(p)
	}

	return 0
}

func (n *Node) Print(depth int) {
	if n.value < 0 {
		for _, child := range n.children {
			child.Print(depth + 1)
		}
	} else {
		for i := 0; i < depth; i++ {
			fmt.Printf("-")
		}
		fmt.Printf("> %v\n", n.value)
	}
}

func split(expr string) []string {
	tokens := []string{}
	token := []rune{}
	depth := 0

	for _, ch := range expr {
		switch ch {
		case '[':
			depth += 1
			if depth > 1 {
				token = append(token, ch)
			}
		case ']':
			depth -= 1
			if depth == 0 && len(token) > 0 {
				tokens = append(tokens, string(token))
				token = []rune{}
			}
			if depth > 0 {
				token = append(token, ch)
			}
		case ',':
			if depth == 1 {
				tokens = append(tokens, string(token))
				token = []rune{}
			}
			if depth > 1 {
				token = append(token, ch)
			}
		default:
			token = append(token, ch)
		}
	}

	return tokens
}

func parse(expr string) *Node {
	parent := NewNode()
	for _, el := range split(expr) {
		node := NewNode()
		integer, err := strconv.Atoi(el)
		if err == nil {
			node.value = integer
		} else {
			node = parse(el)
		}
		parent.children = append(parent.children, node)
	}
	return parent
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	i := 1
	sum := 0

	for scanner.Scan() {
		left := parse(scanner.Text())
		scanner.Scan()
		right := parse(scanner.Text())
		scanner.Scan()

		if left.Compare(right) < 0 {
			sum += i
		}
		i++
	}

	fmt.Println(sum)
}
