package main

import (
	"bufio"
	"fmt"
	"log"
	"strconv"
	"strings"

	"github.com/stevwonder/advent_of_code/2021/v1/utils"
)

func parse(input string) []int {
	var vals []int

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		val, err := strconv.Atoi(scanner.Text())
		if err != nil {
			log.Fatal(err)
		}
		vals = append(vals, val)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return vals
}

func part1(vals []int) (int, error) {
	if len(vals) == 0 {
		return -1, fmt.Errorf("Vals must be longer than 0")
	}

	increases := 0
	for i, v1 := range vals[:len(vals)-1] {
		if vals[i+1] > v1 {
			increases += 1
		}
	}
	return increases, nil
}

func part2(vals []int) (int, error) {
	if len(vals) == 0 {
		return -1, fmt.Errorf("Vals must be longer than 0")
	}

	increases := 0
	for i, v1 := range vals[:len(vals)-3] {
		if vals[i+3] > v1 {
			increases += 1
		}
	}
	return increases, nil
}

func GetVals(test bool) []int {
	input, err := utils.Input(2021, 1, test)
	if err != nil {
		log.Fatalf("Failed getting the input: %s\n", err.Error())
	}
	vals := parse(input)
	return vals
}

func main() {
	vals := GetVals(false)

	val, err := part1(vals)
	if err != nil {
		log.Fatalf("Part 1 failed: %s\n", err.Error())
	}
	fmt.Println(val)

	val, err = part2(vals)
	if err != nil {
		log.Fatalf("Part 2 failed: %s\n", err.Error())
	}
	fmt.Println(val)
}
