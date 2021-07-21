package main

import (
	"bufio"
	"fmt"
	"log"
	"strings"

	"lukechampine.com/advent/utils"
)

type pw_entry struct {
	min, max int
	letter   rune
	pw       string
}

func NewPWEntry(line string) pw_entry {
	var min, max int
	var letter rune
	var pw string
	_, err := fmt.Sscanf(line, "%d-%d %c: %s", &min, &max, &letter, &pw)
	if err != nil {
		log.Fatalf("Failed to parse '%s' into a pw_entry", line)
	}
	return pw_entry{min, max, letter, pw}
}

func parse(input string) []pw_entry {
	var output []pw_entry
	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		output = append(output, NewPWEntry(scanner.Text()))
	}
	return output
}

func check_password1(entry pw_entry) bool {
	count := strings.Count(entry.pw, string(entry.letter))
	return (count >= entry.min) && (count <= entry.max)
}

func part1(vals []pw_entry) (int, error) {
	num_valid := 0
	for _, entry := range vals {
		if check_password1(entry) {
			num_valid += 1
		}
	}
	return num_valid, nil
}

func check_password2(entry pw_entry) bool {
	a := entry.pw[entry.min-1] == byte(entry.letter)
	b := entry.pw[entry.max-1] == byte(entry.letter)
	return (a || b) && !(a && b)
}

func part2(vals []pw_entry) (int, error) {
	num_valid := 0
	for _, entry := range vals {
		if check_password2(entry) {
			num_valid += 1
		}
	}
	return num_valid, nil
}

func main() {
	input := utils.Input(2020, 2)
	vals := parse(input)
	val, err := part1(vals)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Part 1: %v\n", val)

	val, err = part2(vals)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Part 2: %v\n", val)
}
