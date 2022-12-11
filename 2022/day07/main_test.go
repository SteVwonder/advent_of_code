package main

import (
	"log"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParse(t *testing.T) {
	val := GetVals(true)

	log.Printf("Val: %+v", val)

	a, ok := val.Subdirs["a"]
	log.Printf("a: %+v", a)
	assert.True(t, ok)
	assert.Equal(t, 94853, a.Size)
	assert.Equal(t, 3, len(a.Files))
	assert.Equal(t, 1, len(a.Subdirs))

	e, ok := a.Subdirs["e"]
	log.Printf("e: %+v", e)
	assert.True(t, ok)
	assert.Equal(t, 584, e.Size)
	assert.Equal(t, 1, len(e.Files))
	assert.Equal(t, 0, len(e.Subdirs))

	i, ok := e.Files["i"]
	log.Printf("i: %+v", i)
	assert.True(t, ok)
	assert.Equal(t, 584, i.Size)
}

func TestRun(t *testing.T) {
	vals := GetVals(true)

	val, err := part1(vals)
	assert.NoError(t, err)
	assert.Equal(t, 95437, val)

	val, err = part2(vals)
	assert.NoError(t, err)
	assert.Equal(t, 24933642, val)
}
