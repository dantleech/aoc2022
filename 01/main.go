package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strconv"
	"strings"

	"github.com/repeale/fp-go"
)

func main() {
    fmt.Print(determineNumberOfCaloriesCarriedByElfWithMostCalories("data/part1.input"))
    fmt.Print("\n")
    fmt.Print(determineNumberOfCaloriesCarriedByTopThreeElvesWithMostCalories("data/part1.input"))
}

func determineNumberOfCaloriesCarriedByElfWithMostCalories(filename string) int {
	return determineElfWithMostCalories(readFile(filename))
}

func readFile(filename string) string {
	data, err := ioutil.ReadFile(filename)

	if err != nil {
		panic(err)
	}

    return string(data)
}

func determineNumberOfCaloriesCarriedByTopThreeElvesWithMostCalories(filename string) int {
    input := readFile(filename)
    totals := totals(input)
    sort.Slice(totals, func(a, b int) bool {
        return totals[a] < totals[b]
    })
    topThree := totals[len(totals)-3:]
    return fp.Reduce(func (a, c int) int {return a+c}, 0)(topThree)
}

func determineElfWithMostCalories(input string) int {
    totals := totals(input)

    bestTotal := 0
    for _, total := range totals {
        if bestTotal == 0 || total > bestTotal {
            bestTotal = total
        }

    }
	return bestTotal
}

func totals(input string) []int {
	elfs := strings.Split(input, "\n\n")
	totals := []int{}

	for _, rawElf := range elfs {
		calories := strings.Split(rawElf, "\n")
		total := 0
		for _, calory := range calories {
			if calory == "" {
				continue
			}
			number, err := strconv.Atoi(calory)
			if err != nil {
				panic(err)
			}
			total += number
		}
		totals = append(totals, total)
	}
	return totals
}
