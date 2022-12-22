package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParse(t *testing.T) {
	val := GetVals(true)

	assert.Equal(t, 5, len(val.Trees))
	for _, x := range val.Trees {
		assert.Equal(t, 5, len(x))
	}
	assert.Equal(t, []int{3, 0, 3, 7, 3}, val.Trees[0])
}

func TestRun(t *testing.T) {
	vals := GetVals(true)

	val, err := part1(vals)
	assert.NoError(t, err)
	assert.Equal(t, 21, val)

	val, err = part2(vals)
	assert.NoError(t, err)
	assert.Equal(t, 8, val)
}
