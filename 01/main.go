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
    fmt.Print(part1("data/part1.input"))
    fmt.Print("\n")
    fmt.Print(part2("data/part1.input"))
    fmt.Print("\n")
}

func part1(filename string) int {
    return topElves(filename, 1)
}

func part2(filename string) int {
    return topElves(filename, 3)
}

func topElves(filename string, count int) int {
    input := readFile(filename)
    totals := totals(input)
    sort.Slice(totals, func(a, b int) bool {
        return totals[a] < totals[b]
    })
    topThree := totals[len(totals)-count:]
    return fp.Reduce(func (a, c int) int {return a+c}, 0)(topThree)
}

func readFile(filename string) string {
	data, err := ioutil.ReadFile(filename)

	if err != nil {
		panic(err)
	}

    return string(data)
}

func totals(input string) []int {
    return fp.Map(func (s string) int {
        return fp.Reduce(func (ac int, v string) int {
            if "" == v {
                return ac
            }
            int, err := strconv.Atoi(v)
            if err != nil {
                panic(err)
            }
            ac += int
            return ac
        }, 0)(strings.Split(s, "\n"))
    })(strings.Split(input, "\n\n"))
}
