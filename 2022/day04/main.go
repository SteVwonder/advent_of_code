package main

import (
	"bufio"
	"fmt"
	"log"
	"strconv"
	"strings"

	"github.com/stevwonder/advent_of_code/2022/v1/utils"
)

type ValType [][]Range

type Range struct {
	Start int
	End   int
}

func (r *Range) Superset(other *Range) bool {
	return r.Start <= other.Start && r.End >= other.End
}

func (r *Range) Overlaps(other *Range) bool {
	return (other.Start >= r.Start && other.Start <= r.End) ||
		(other.End >= r.Start && other.End <= r.End)
}

func stringToRange(in string) (Range, error) {
	outRange := Range{}
	splits := strings.Split(in, "-")
	if len(splits) != 2 {
		return outRange, fmt.Errorf("Wrong number of ints in range: %s", in)
	}

	ints := []int{}
	for _, x := range splits {
		parsedInt, err := strconv.Atoi(x)
		if err != nil {
			return outRange, fmt.Errorf("Invalid int (%s) in range: %s", x, in)
		}
		ints = append(ints, parsedInt)
	}

	return Range{
		Start: ints[0],
		End:   ints[1],
	}, nil
}

func parse(input string) ValType {
	var vals ValType
	lineNum := 1

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		splits := strings.Split(line, ",")
		if len(splits) != 2 {
			log.Fatalf("Too many ranges on line %d", lineNum)
		}
		rangePair := []Range{}
		for _, x := range splits {
			parsedRange, err := stringToRange(x)
			if err != nil {
				log.Fatal(err.Error())
			}
			rangePair = append(rangePair, parsedRange)
		}
		lineNum += 1
		vals = append(vals, rangePair)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return vals
}

func part1(vals ValType) (int, error) {
	if len(vals) == 0 {
		return -1, fmt.Errorf("Vals must be longer than 0")
	}

	numSuperSets := 0
	for _, rangePair := range vals {
		if rangePair[0].Superset(&rangePair[1]) || rangePair[1].Superset(&rangePair[0]) {
			numSuperSets += 1
		}
	}

	return numSuperSets, nil
}

func part2(vals ValType) (int, error) {
	if len(vals) == 0 {
		return -1, fmt.Errorf("Vals must be longer than 0")
	}

	numOverlaps := 0
	for _, rangePair := range vals {
		if rangePair[0].Superset(&rangePair[1]) ||
			rangePair[1].Superset(&rangePair[0]) ||
			rangePair[0].Overlaps(&rangePair[1]) {
			numOverlaps += 1
		}
	}

	return numOverlaps, nil
}

func GetVals(test bool) ValType {
	input, err := utils.Input(2022, 4, test)
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
