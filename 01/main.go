package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
    fmt.Print(determineElfWithMostCaloriesFromFile("data/part1.input"))
}

func determineElfWithMostCalories(input string) int {
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

    bestTotal := 0
    for _, total := range totals {
        if bestTotal == 0 || total > bestTotal {
            bestTotal = total
        }

    }
	return bestTotal
}

func determineElfWithMostCaloriesFromFile(filename string) int {
	data, err := ioutil.ReadFile(filename)

    if err != nil {
        panic(err)
    }

	return determineElfWithMostCalories(string(data))
}
