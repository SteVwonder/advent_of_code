package main

import (
	"testing"

	"github.com/gammazero/deque"
	"github.com/stretchr/testify/assert"
)

func SliceFromDeque[T any](in *deque.Deque[T]) []T {
	out := make([]T, in.Len())
	for i := 0; i < in.Len(); i++ {
		out[i] = in.At(i)
	}
	return out
}

func TestParse(t *testing.T) {
	val := GetVals(true)

	if assert.Equal(t, 3, len(val.Stacks)) {
		assert.Equal(t, []string{"Z", "N"}, SliceFromDeque(val.Stacks[0]))
		assert.Equal(t, []string{"M", "C", "D"}, SliceFromDeque(val.Stacks[1]))
		assert.Equal(t, []string{"P"}, SliceFromDeque(val.Stacks[2]))
	}
	if assert.Equal(t, 4, len(val.Instructions)) {
		assert.Equal(t, Instruction{1, 2, 1}, val.Instructions[0])
		assert.Equal(t, Instruction{3, 1, 3}, val.Instructions[1])
		assert.Equal(t, Instruction{2, 2, 1}, val.Instructions[2])
		assert.Equal(t, Instruction{1, 1, 2}, val.Instructions[3])
	}
}

func TestRun(t *testing.T) {
	vals := GetVals(true)

	val, err := part1(vals)
	assert.NoError(t, err)
	assert.Equal(t, "CMZ", val)

	vals = GetVals(true)
	val, err = part2(vals)
	assert.NoError(t, err)
	assert.Equal(t, "MCD", val)
}
