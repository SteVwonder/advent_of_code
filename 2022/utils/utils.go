package utils

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"path/filepath"
	"sort"
	"time"

	"golang.org/x/exp/constraints"

	"github.com/integralist/go-findroot/find"
)

// Input returns the input for the specified year and day as a string,
// downloading it if it does not already exist on disk.
// Original from: https://github.com/lukechampine/advent/blob/master/utils/utils.go
func Input(year int, day int, test bool) (string, error) {
	if test {
		filename := "test-input"
		if _, err := os.Stat(filename); err != nil {
			return "", err
		} else {
			return ReadInput(filename)
		}
	}

	filename := fmt.Sprintf("day%v_input.txt", day)
	if _, err := os.Stat(filename); err != nil {
		est, err := time.LoadLocation("EST")
		if err != nil {
			return "", err
		}
		if t := time.Date(year, time.December, day, 0, 0, 0, 0, est); time.Until(t) > 0 {
			fmt.Printf("Puzzle not unlocked yet! Sleeping for %v...\n", time.Until(t))
			time.Sleep(time.Until(t) + 3*time.Second) // don't want to fire too early
		}

		repoRoot := ""
		cookie := os.Getenv("AOC_USER_ID")
		if cookie == "" {
			root, err := find.Repo()
			if err == nil {
				repoRoot = root.Path
				bytes, err := ioutil.ReadFile(filepath.Join(root.Path, ".aoc-cookie"))
				if err == nil {
					cookie = string(bytes)
				}
			}
		}
		if cookie == "" {
			return "", fmt.Errorf("AOC session cookie not found. Either set AOC_USER_ID or create %s", filepath.Join(repoRoot, ".aoc-cookie"))
		}

		fmt.Println("Downloading input...")
		req, _ := http.NewRequest(http.MethodGet, fmt.Sprintf("https://adventofcode.com/%v/day/%v/input", year, day), nil)
		req.AddCookie(&http.Cookie{
			Name:  "session",
			Value: cookie,
		})
		resp, err := http.DefaultClient.Do(req)
		if err != nil {
			return "", err
		}
		defer resp.Body.Close()
		data, err := ioutil.ReadAll(resp.Body)
		if err != nil {
			return "", err
		}
		if err := ioutil.WriteFile(filename, data, 0660); err != nil {
			return "", err
		}
	}
	return ReadInput(filename)
}

// ReadInput returns the contents of filename as a string.
// Original from: https://github.com/lukechampine/advent/blob/master/utils/utils.go
func ReadInput(filename string) (string, error) {
	data, err := ioutil.ReadFile(filename)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

func Max[T constraints.Ordered](arg ...T) T {
	var curr_max T = arg[0]
	for _, x := range arg[1:] {
		if x > curr_max {
			curr_max = x
		}
	}
	return curr_max
}

func Min[T constraints.Ordered](arg ...T) T {
	var curr_min T = arg[0]
	for _, x := range arg[1:] {
		if x < curr_min {
			curr_min = x
		}
	}
	return curr_min
}

func Sum[T constraints.Integer](s []T) T {
	var accum T
	for _, val := range s {
		accum += val
	}
	return accum
}

func Map[X any, Y any](in []X, f func(X) (Y, error)) ([]Y, error) {
	out := make([]Y, len(in))
	for i, x := range in {
		y, err := f(x)
		if err != nil {
			return out, err
		}
		out[i] = y
	}
	return out, nil
}

func Reversed[T comparable](s []T) []T {
	out := make([]T, len(s))
	copy(out, s)
	sort.SliceStable(out, func(i, j int) bool {
		return i > j
	})
	return out
}
