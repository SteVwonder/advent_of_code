package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"strconv"
	"strings"

	"github.com/stevwonder/advent_of_code/2022/v1/utils"
)

type ValType = *Directory
type AnswerType = int

type Directory struct {
	Name    string
	Parent  *Directory
	Subdirs map[string]*Directory
	Size    int
	Files   map[string]File
}

type File struct {
	Name string
	Size int
}

func NewDirectory(name string, parent *Directory) *Directory {
	return &Directory{
		Name:    name,
		Parent:  parent,
		Subdirs: make(map[string]*Directory),
		Files:   make(map[string]File),
		Size:    0,
	}
}

func (d *Directory) Cd(path string) (*Directory, error) {
	if path == ".." {
		if d.Parent == nil {
			return nil, fmt.Errorf("Root dir / has no parent")
		}
		return d.Parent, nil
	}
	if strings.ContainsRune(path, '/') {
		return nil, fmt.Errorf("Cannot Cd to absolute paths or deeper than one dir at a time")
	}
	if subDir, ok := d.Subdirs[path]; ok {
		return subDir, nil
	}
	return nil, fmt.Errorf("Subdirectory not found.  Cannot cd to %s from %s", path, d.Name)
}

func (d *Directory) AbsPath() string {
	if d.Name == "" && d.Parent == nil {
		return "/"
	}

	path := d.Name

	currDir := d.Parent
	for currDir != nil {
		path = currDir.Name + "/" + path
		currDir = currDir.Parent
	}
	return path
}

func (d *Directory) AddFile(file File) {
	d.Files[file.Name] = file

	currDir := d
	for currDir != nil {
		currDir.Size += file.Size
		currDir = currDir.Parent
	}
}

func (d *Directory) NewSubDir(name string) {
	d.Subdirs[name] = NewDirectory(name, d)
}

func (d *Directory) Walk(walkFn func(*Directory) error) error {
	err := walkFn(d)
	if err != nil {
		return err
	}
	for _, subdir := range d.Subdirs {
		err := subdir.Walk(walkFn)
		if err != nil {
			return err
		}
	}
	return nil
}

func parse(input string) ValType {
	lineNum := 0
	rootDir := NewDirectory("", nil)
	currDir := rootDir
	inLs := false

	scanner := bufio.NewScanner(strings.NewReader(input))
	// Parse the stacks
	for scanner.Scan() {
		line := scanner.Text()
		lineNum += 1

		if lineNum == 1 {
			if line != "$ cd /" {
				log.Fatalf("Unexpected first command: %s", line)
			}
			continue
		}

		if line[0] == '$' { // Parse command
			inLs = false
			switch line[2:4] {
			case "ls":
				inLs = true
			case "cd":
				d, err := currDir.Cd(line[5:])
				currDir = d
				if err != nil {
					log.Fatalf("Cd error on line %d: %s", lineNum, err.Error())
				}
			default:
				log.Fatalf("Unknown command %s on line %d", line[2:4], lineNum)
			}
		} else {
			if !inLs {
				log.Fatal("Not expected ls output but no command found")
			}
			splits := strings.Split(line, " ")
			if splits[0] == "dir" {
				currDir.NewSubDir(splits[1])
			} else if i, err := strconv.Atoi(splits[0]); err == nil {
				currDir.AddFile(File{splits[1], i})
			} else {
				log.Fatalf("Unknown ls output on line %d", lineNum)
			}
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return rootDir
}

func part1(val ValType) (AnswerType, error) {
	sizeSum := 0
	err := val.Walk(func(dir *Directory) error {
		if dir.Size <= 100000 {
			sizeSum += dir.Size
		}
		return nil
	})
	if err != nil {
		return 0, err
	}
	return sizeSum, nil
}

func part2(val ValType) (AnswerType, error) {
	currentUsedSpace := val.Size
	targetSpaceToFree := 30000000 - (70000000 - currentUsedSpace)

	size := 0
	currMinDiff := math.MaxInt
	err := val.Walk(func(dir *Directory) error {
		diff := dir.Size - targetSpaceToFree
		if diff > 0 && diff < currMinDiff {
			currMinDiff = diff
			size = dir.Size
		}
		return nil
	})
	if err != nil {
		return 0, err
	}
	return size, nil
}

func GetVals(test bool) ValType {
	input, err := utils.Input(2022, 7, test)
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
