package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRun(t *testing.T) {
	vals := GetVals(true)

	val, err := part1(vals)
	assert.NoError(t, err)
	assert.Equal(t, 24000, val)

	val, err = part2(vals)
	assert.NoError(t, err)
	assert.Equal(t, 45000, val)
}
