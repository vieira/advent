package main

import (
	"bufio"
	"fmt"
	"os"
	"unicode"
)

type Empty struct{}
type Set[T comparable] struct {
	bag map[T]Empty
}

func NewSet[T comparable](items []T) *Set[T] {
	s := &Set[T]{bag: make(map[T]Empty, len(items))}
	s.AddMany(items)
	return s
}

func (s *Set[T]) Has(item T) bool {
	_, exists := s.bag[item]
	return exists
}

func (s *Set[T]) Add(item T) bool {
	if s.Has(item) {
		return false
	}
	s.bag[item] = Empty{}
	return true
}

func (s *Set[T]) AddMany(items []T) {
	for _, c := range items {
		s.Add(c)
	}
}

func (s *Set[T]) Items() []T {
	keys := make([]T, len(s.bag))
	i := 0
	for k := range s.bag {
		keys[i] = k
	}
	return keys
}

func (s *Set[T]) Intersection(t *Set[T]) *Set[T] {
	r := NewSet[T]([]T{})
	for c := range t.bag {
		if s.Has(c) {
			r.Add(c)
		}
	}
	return r
}

func priority(item rune) int {
	if unicode.IsLower(item) {
		return int(item) - int('a') + 1
	}
	return int(item) - int('A') + 27
}

func badge(group []*Set[rune]) rune {
	i := 0
	intersection := group[i]
	for i = 1; i < len(group); i++ {
		intersection = intersection.Intersection(group[i])
	}
	return intersection.Items()[0]
}

func main() {
	var group []*Set[rune]
	scanner := bufio.NewScanner(os.Stdin)
	total := 0

	for scanner.Scan() {
		line := scanner.Text()
		group = append(group, NewSet[rune]([]rune(line)))

		if len(group) >= 3 {
			total += priority(badge(group))
			group = []*Set[rune]{}
		}
	}
	fmt.Println(total)
}
