package main

import (
	"bufio"
	"fmt"
	"log"
	"sort"
	"strconv"
	"strings"

	"github.com/stevwonder/advent_of_code/2022/v1/utils"
)

func parse(input string) []int {
	var vals []int

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)
		val := -1
		if line != "" {
			var err error
			val, err = strconv.Atoi(line)
			if err != nil {
				log.Fatalf("%s", err.Error())
			}
		}
		vals = append(vals, val)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return vals
}

func getElves(vals []int) []int {
	elves := []int{}
	curr_calories := 0
	for _, val := range vals {
		if val == -1 {
			elves = append(elves, curr_calories)
			curr_calories = 0
		} else {
			curr_calories += val
		}
	}
	elves = append(elves, curr_calories)
	return elves
}

func part1(vals []int) (int, error) {
	if len(vals) == 0 {
		return -1, fmt.Errorf("Vals must be longer than 0")
	}

	elves := getElves(vals)
	return utils.Max(elves...), nil
}

func part2(vals []int) (int, error) {
	if len(vals) == 0 {
		return -1, fmt.Errorf("Vals must be longer than 0")
	}

	elves := getElves(vals)
	sort.Ints(elves)
	top3 := elves[len(elves)-3 : len(elves)]
	sum := 0
	for _, i := range top3 {
		sum += i
	}
	return sum, nil
}

func GetVals(test bool) []int {
	input, err := utils.Input(2022, 1, test)
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
