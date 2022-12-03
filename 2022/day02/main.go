package main

import (
	"bufio"
	"fmt"
	"log"
	"strings"

	"github.com/stevwonder/advent_of_code/2022/v1/utils"
)

var winningMove = map[string]string{
	"A": "Y",
	"B": "Z",
	"C": "X",
}

var tieingMove = map[string]string{
	"A": "X",
	"B": "Y",
	"C": "Z",
}

var losingMove = map[string]string{
	"A": "Z",
	"B": "X",
	"C": "Y",
}

var movePoints = map[string]int{
	"X": 1,
	"Y": 2,
	"Z": 3,
}

func parse(input string) [][]string {
	var vals [][]string
	lineNum := 1

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		splits := strings.Split(line, " ")
		if len(splits) != 2 {
			log.Fatalf("Invalid number of moves (%d) on line %d", len(splits), lineNum)
		}
		if _, ok := winningMove[splits[0]]; !ok {
			log.Fatalf("%s is not a valid move for the opponent. Line %d", splits[0], lineNum)
		}
		if _, ok := movePoints[splits[1]]; !ok {
			log.Fatalf("%s is not a valid move for me. Line %d", splits[1], lineNum)
		}
		lineNum += 1
		vals = append(vals, splits)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return vals
}

func part1(vals [][]string) (int, error) {
	if len(vals) == 0 {
		return -1, fmt.Errorf("Vals must be longer than 0")
	}

	totalScore := 0
	for _, moves := range vals {
		// Win vs tie vs lose score component
		if moves[1] == tieingMove[moves[0]] {
			totalScore += 3
		} else if moves[1] == winningMove[moves[0]] {
			totalScore += 6
		}

		// My move score
		totalScore += movePoints[moves[1]]
	}

	return totalScore, nil
}

func part2(vals [][]string) (int, error) {
	if len(vals) == 0 {
		return -1, fmt.Errorf("Vals must be longer than 0")
	}

	totalScore := 0
	for _, moves := range vals {
		switch moves[1] {
		case "X": // Lose
			totalScore += movePoints[losingMove[moves[0]]]
		case "Y": // Draw
			totalScore += movePoints[tieingMove[moves[0]]] + 3
		case "Z": // Win
			totalScore += movePoints[winningMove[moves[0]]] + 6
		default:
			return 0, fmt.Errorf("Unknown target state")
		}
	}

	return totalScore, nil
}

func GetVals(test bool) [][]string {
	input, err := utils.Input(2022, 2, test)
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
