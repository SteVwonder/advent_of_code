package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strconv"
)

func parse(filename string) []int {
	file, err := os.Open(filename)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var vals []int

	scanner := bufio.NewScanner(file)
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
	for i, v1 := range vals {
		for _, v2 := range vals[i+1:] {
			if v1+v2 == 2020 {
				return (v1 * v2), nil
			}
		}
	}

	return -1, errors.New("failed to find vals that sum to 2020")
}

func part2(vals []int) (int, error) {
	for i, v1 := range vals {
		for j, v2 := range vals[i+1:] {
			for _, v3 := range vals[j+1:] {
				if v1+v2+v3 == 2020 {
					return (v1 * v2 * v3), nil
				}
			}
		}
	}
	return -1, errors.New("failed to find vals that sum to 2020")
}

func main() {
	input := parse("./input")
	val, err := part1(input)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(val)

	val, err = part2(input)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(val)
}
