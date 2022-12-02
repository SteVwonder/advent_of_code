package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRun(t *testing.T) {
	vals := GetVals(true)

	val, err := part1(vals)
	assert.NoError(t, err)
	assert.Equal(t, val, 24000)

	val, err = part2(vals)
	assert.NoError(t, err)
	assert.Equal(t, val, 5)
}
