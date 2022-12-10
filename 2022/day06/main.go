package main

import (
	"bufio"
	"fmt"
	"log"
	"strings"

	"github.com/stevwonder/advent_of_code/2022/v1/utils"
)

type ValType = string
type AnswerType = int

func parse(input string) ValType {
	var val ValType

	scanner := bufio.NewScanner(strings.NewReader(input))
	// Parse the stacks
	if !scanner.Scan() {
		log.Fatal("")
	}
	val = scanner.Text()

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return val
}

type Counter struct {
	Map map[byte]int
}

func NewCounter() Counter {
	out := Counter{}
	out.Map = make(map[byte]int)
	return out
}

func (c *Counter) Add(in byte) {
	val, _ := c.Map[in]
	c.Map[in] = val + 1
}

func (c *Counter) Remove(out byte) {
	val, ok := c.Map[out]
	if !ok {
		log.Fatalf("Counter does not contain byte for removal: %s", string(out))
	}
	c.Map[out] = val - 1
}

func (c *Counter) Unique() bool {
	for _, x := range c.Map {
		if x != 0 && x != 1 {
			return false
		}
	}
	return true
}

func part1(val ValType) (AnswerType, error) {
	counter := NewCounter()

	for i := 0; i < 4; i++ {
		counter.Add(val[i])
	}
	if counter.Unique() {
		return 4, nil
	}

	for i, x := range val[4:] {
		counter.Remove(val[i])
		counter.Add(byte(x))
		if counter.Unique() {
			return i + 5, nil
		}
	}

	return 0, fmt.Errorf("No sequence found")
}

func part2(val ValType) (AnswerType, error) {
	counter := NewCounter()

	for i := 0; i < 14; i++ {
		counter.Add(val[i])
	}
	if counter.Unique() {
		return 14, nil
	}

	for i, x := range val[14:] {
		counter.Remove(val[i])
		counter.Add(byte(x))
		if counter.Unique() {
			return i + 15, nil
		}
	}

	return 0, fmt.Errorf("No sequence found")
}

func GetVals(test bool) ValType {
	input, err := utils.Input(2022, 6, test)
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

	vals = GetVals(false)
	val, err = part2(vals)
	if err != nil {
		log.Fatalf("Part 2 failed: %s\n", err.Error())
	}
	fmt.Println(val)
}
