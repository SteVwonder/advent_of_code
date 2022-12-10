package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRun(t *testing.T) {
	for _, testSetup := range []struct {
		Input     string
		Expected1 int
		Expected2 int
	}{
		{"mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19},
		{"bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23},
		{"nppdvjthqldpwncqszvftbrmjlhg", 6, 23},
		{"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29},
		{"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26},
	} {
		val, err := part1(testSetup.Input)
		assert.NoError(t, err)
		assert.Equal(t, testSetup.Expected1, val)

		val, err = part2(testSetup.Input)
		assert.NoError(t, err)
		assert.Equal(t, testSetup.Expected2, val)
	}
}
