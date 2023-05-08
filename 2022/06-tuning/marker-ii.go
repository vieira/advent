package main

import (
	"bufio"
	"fmt"
	"os"
)

const markerSize = 14

func ScanMarkers(data []byte, atEOF bool) (advance int, mk []byte, err error) {
	if data[len(data)-1] == '\n' {
		data = data[:len(data)-2]
	}
	if len(data) >= markerSize {
		return 1, data[:markerSize], nil
	}
	if atEOF {
		return len(data), nil, nil
	}
	// Request more data.
	return 0, nil, nil
}

func isMarker(str string) bool {
	cs := map[rune]int{}
	for i, c := range str {
		if _, exists := cs[c]; exists {
			return false
		}
		cs[c] = i
	}
	return true
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(ScanMarkers)
	i := markerSize

	for scanner.Scan() {
		c := scanner.Text()
		if isMarker(c) {
			fmt.Println(i)
			break;
		}
		i += 1
	}
}
