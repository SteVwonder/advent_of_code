package main

import (
	"bufio"
	"fmt"
	"log"
	"strings"

	mapset "github.com/deckarep/golang-set"
	"github.com/stevwonder/advent_of_code/2022/v1/utils"
)

type ValType []RuckSack

type RuckSack struct {
	Total        mapset.Set
	CompartmentA mapset.Set
	CompartmentB mapset.Set
}

func stringToSet(in string) mapset.Set {
	outSet := mapset.NewSet()
	for _, x := range in {
		outSet.Add(x)
	}
	return outSet
}

func parse(input string) ValType {
	var vals []RuckSack
	lineNum := 1

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		length := len(line)
		if length%2 == 1 {
			log.Fatalf("Invalid number of items (%d) on line %d", length, lineNum)
		}
		sack := RuckSack{
			Total:        stringToSet(line),
			CompartmentA: stringToSet(line[0 : length/2]),
			CompartmentB: stringToSet(line[length/2 : length]),
		}
		lineNum += 1
		vals = append(vals, sack)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return vals
}

func getPriority(item byte) int {
	if item >= 97 && item <= 122 {
		return int(item - 96) // 1 -> 26
	} else if item >= 65 && item <= 90 {
		return int(item - 65 + 27) // 27 -> 52
	} else {
		log.Fatalf("Invalid item value (%d)", int(item))
	}
	return -1
}

func part1(vals ValType) (int, error) {
	if len(vals) == 0 {
		return -1, fmt.Errorf("Vals must be longer than 0")
	}

	prioritySum := 0
	for _, rucksack := range vals {
		common := rucksack.CompartmentA.Intersect(rucksack.CompartmentB)
		if common.Cardinality() != 1 {
			log.Fatalf("RuckSack %v doesn't have 1 common element, it has %d", rucksack, common.Cardinality())
		}
		item := common.ToSlice()[0]
		itemRune := item.(rune)
		prioritySum += getPriority(byte(itemRune))
	}

	return prioritySum, nil
}

func part2(vals ValType) (int, error) {
	if len(vals) == 0 {
		return -1, fmt.Errorf("Vals must be longer than 0")
	}

	elfGroups := [][]RuckSack{}
	currGroup := []RuckSack{}
	for i, rucksack := range vals {
		if i%3 == 0 && i > 0 {
			elfGroups = append(elfGroups, currGroup)
			currGroup = []RuckSack{}
		}
		currGroup = append(currGroup, rucksack)
	}
	elfGroups = append(elfGroups, currGroup)

	prioritySum := 0
	for _, elfGroup := range elfGroups {
		if len(elfGroup) != 3 {
			return 0, fmt.Errorf("elfGroup %v only contains %d elves", elfGroup, len(elfGroup))
		}
		common := elfGroup[0].Total.Intersect(elfGroup[1].Total).Intersect(elfGroup[2].Total)
		if common.Cardinality() != 1 {
			log.Fatalf("elfGroup %v doesn't have 1 common element, it has %d", elfGroup, common.Cardinality())
		}
		item := common.ToSlice()[0]
		itemRune := item.(rune)
		prioritySum += getPriority(byte(itemRune))
	}

	return prioritySum, nil
}

func GetVals(test bool) ValType {
	input, err := utils.Input(2022, 3, test)
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
