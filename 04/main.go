package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"

	"github.com/samber/lo"
)

func main() {
    fmt.Printf("Part 1: %d\n", numberOfullyOverlappingPairs("data/input"))
}

type SectorRange struct {
    start int;
    end int;
}

func (r1 SectorRange) contains(r2 SectorRange) bool {
    return r2.start >= r1.start && r2.end <= r1.end
}

type Pair struct {
    range1 SectorRange;
    range2 SectorRange;
}

func (p Pair) overlap() bool {
    return p.range1.contains(p.range2) || p.range2.contains(p.range1)
}

func numberOfullyOverlappingPairs(filename string) int {
    return lo.Reduce(sectorRanges(filename), func (agg int, p Pair, _ int) int {
        if p.overlap() {
            agg += 1
        }

        return agg
    }, 0)
}

func sectorRanges(filename  string) []Pair {
    return lo.Map(
        strings.Split(readInput(filename), "\n"), 
        func (line string, index int) Pair {

            return (func (ranges []SectorRange) Pair {
                return Pair{
                    range1: ranges[0],
                    range2: ranges[1],
                }
            })(lo.Map(
                lo.Map(
                    strings.Split(line, ","),
                    func (stringRange string, _ int) []string {
                        return strings.Split(stringRange, "-")
                    },
                ),
                func (stringRanges []string, _ int) SectorRange {
                    return SectorRange{
                        start: strToInt(stringRanges[0]),
                        end: strToInt(stringRanges[1]),
                    }
                },
            ))
        },
    )
}

func strToInt(string string) int {
    int, err := strconv.Atoi(string)
    if err != nil {
        panic(err)
    }
    return int
}

func readInput(filename string) string {
    data, err := ioutil.ReadFile(filename)

    if err != nil {
        panic(err)
    }

    return strings.TrimSpace(string(data))
}
