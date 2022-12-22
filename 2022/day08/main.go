package main

import (
	"bufio"
	"fmt"
	"log"
	"strconv"
	"strings"

	"github.com/stevwonder/advent_of_code/2022/v1/utils"
)

type ValType = *Grid
type AnswerType = int

type Grid struct {
	Trees [][]int
}

func NewGrid() *Grid {
	out := Grid{}
	out.Trees = [][]int{}
	return &out
}

func parse(input string) ValType {
	lineNum := 0
	grid := NewGrid()

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		lineNum += 1
		characters := strings.SplitAfter(line, "")
		treeRow, err := utils.Map(characters, func(in string) (int, error) {
			return strconv.Atoi(in)
		})
		if err != nil {
			log.Fatalf("Error in Map: %s", err.Error())
		}
		grid.Trees = append(grid.Trees, treeRow)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return grid
}

// Returns the line of sight starting from the tree at rowIdx, colIdx and moving outwards
func getLinesOfSight(grid *Grid, rowIdx int, colIdx int) ([]int, []int, []int, []int) {
	west := utils.Reversed(grid.Trees[rowIdx][0:colIdx])
	east := grid.Trees[rowIdx][colIdx+1:]
	north := []int{}
	for _, row := range grid.Trees[:rowIdx] {
		north = append(north, row[colIdx])
	}
	north = utils.Reversed(north)
	south := []int{}
	for _, row := range grid.Trees[rowIdx+1:] {
		south = append(south, row[colIdx])
	}
	return north, south, east, west
}

func visible(grid *Grid, rowIdx int, colIdx int) bool {
	treeHeight := grid.Trees[rowIdx][colIdx]

	north, south, east, west := getLinesOfSight(grid, rowIdx, colIdx)
	maxes := []int{
		utils.Max(west...),
		utils.Max(east...),
		utils.Max(north...),
		utils.Max(south...),
	}

	minOfMaxes := utils.Min(maxes...)
	return treeHeight > minOfMaxes
}

func part1(val ValType) (AnswerType, error) {
	numVisible := 0
	trees := val.Trees
	numVisible += (len(trees) * 2) + ((len(trees[0]) - 2) * 2)

	for r, row := range trees[1 : len(trees)-1] {
		rowIdx := r + 1
		for c, _ := range row[1 : len(row)-1] {
			colIdx := c + 1
			visible := visible(val, rowIdx, colIdx)
			if visible {
				numVisible += 1
			}
		}
	}
	return numVisible, nil
}

func sightScoreOneDir(lineOfSight []int, treeHeight int) int {
	for i, x := range lineOfSight {
		if x >= treeHeight {
			return i + 1
		}
	}
	return len(lineOfSight)
}

func sightScore(grid *Grid, rowIdx int, colIdx int) int {
	score := 1
	treeHeight := grid.Trees[rowIdx][colIdx]

	north, south, east, west := getLinesOfSight(grid, rowIdx, colIdx)
	linesOfSight := [][]int{north, south, east, west}
	for _, los := range linesOfSight {
		score *= sightScoreOneDir(los, treeHeight)
	}

	return score
}

func part2(val ValType) (AnswerType, error) {
	score := 0
	trees := val.Trees

	for r, row := range trees[1 : len(trees)-1] {
		rowIdx := r + 1
		for c, _ := range row[1 : len(row)-1] {
			colIdx := c + 1
			score = utils.Max(score, sightScore(val, rowIdx, colIdx))
		}
	}

	return score, nil
}

func GetVals(test bool) ValType {
	input, err := utils.Input(2022, 8, test)
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
