package main

import (
	"bufio"
	"fmt"
	"log"
	"regexp"
	"strconv"
	"strings"

	"github.com/gammazero/deque"
	"github.com/stevwonder/advent_of_code/2022/v1/utils"
)

type ValType Input
type AnswerType = string

type Instruction struct {
	Num  int
	From int
	To   int
}

type Stacks []*deque.Deque[string]

type Input struct {
	Stacks       Stacks
	Instructions []Instruction
}

func parse(input string) ValType {
	var val ValType
	lineNum := 0
	numStacks := 0

	scanner := bufio.NewScanner(strings.NewReader(input))
	// Parse the stacks
	for scanner.Scan() {
		lineNum += 1
		line := scanner.Bytes()
		if string(line[0:2]) == " 1" {
			break // final line of the stacks
		}
		if lineNum == 1 {
			numStacks = (len(line) / 4) + 1
			for i := 0; i < numStacks; i++ {
				val.Stacks = append(val.Stacks, deque.New[string]())
			}
		}

		for stackID, stack := range val.Stacks {
			stackOffset := stackID * 4
			chunk := line[0+stackOffset : 3+stackOffset]
			chunkVal := string(chunk[1])
			if chunkVal != " " {
				stack.PushFront(chunkVal)
			}
		}
	}

	// skip empty newline after stacks
	scanner.Scan()
	lineNum += 1
	if scanner.Text() != "" {
		log.Fatalf("Expected newline not found after stacks on line %d", lineNum)
	}

	instructionRE := regexp.MustCompile(`move (\d+) from (\d+) to (\d+)`)
	// Parse the instructions
	for scanner.Scan() {
		line := scanner.Text()
		matches := instructionRE.FindStringSubmatch(line)
		if len(matches) != 4 {
			log.Printf("Matches: %v", matches)
			log.Fatalf("Failure to parse instruction on line %d: %s", lineNum, line)
		}
		matchesInt := make([]int, len(matches)-1)
		for i, match := range matches[1:] {
			matchInt, err := strconv.Atoi(match)
			if err != nil {
				log.Fatalf("Invalid int in instruction on line %d: %s", lineNum, line)
			}
			matchesInt[i] = matchInt
		}
		if matchesInt[1] > numStacks || matchesInt[2] > numStacks {
			log.Fatalf("Invalid stack ID in instruction on line %d: %s", lineNum, line)
		}
		val.Instructions = append(val.Instructions, Instruction{matchesInt[0], matchesInt[1], matchesInt[2]})
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return val
}

func (s Stacks) execInstructionPart1(inst Instruction) error {
	for i := 0; i < inst.Num; i++ {
		curr := s[inst.From-1].PopBack()
		s[inst.To-1].PushBack(curr)
	}
	return nil
}

func (s Stacks) execInstructionPart2(inst Instruction) error {
	moving := deque.New[string](inst.Num)
	for i := 0; i < inst.Num; i++ {
		curr := s[inst.From-1].PopBack()
		moving.PushFront(curr)
	}
	for i := 0; i < inst.Num; i++ {
		curr := moving.PopFront()
		s[inst.To-1].PushBack(curr)
	}
	return nil
}

func part1(val ValType) (AnswerType, error) {
	for _, instruction := range val.Instructions {
		err := val.Stacks.execInstructionPart1(instruction)
		if err != nil {
			return "", err
		}
	}
	output := []string{}
	for _, stack := range val.Stacks {
		output = append(output, stack.Back())
	}

	return strings.Join(output, ""), nil
}

func part2(val ValType) (AnswerType, error) {
	for _, instruction := range val.Instructions {
		err := val.Stacks.execInstructionPart2(instruction)
		if err != nil {
			return "", err
		}
	}
	output := []string{}
	for _, stack := range val.Stacks {
		output = append(output, stack.Back())
	}

	return strings.Join(output, ""), nil
}

func GetVals(test bool) ValType {
	input, err := utils.Input(2022, 5, test)
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
