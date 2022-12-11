package main

import (
	"io/ioutil"
	"strconv"
	"strings"

	"github.com/davecgh/go-spew/spew"
	"github.com/samber/lo"
)

type SectorRange struct {
    start int;
    end int;
}

type Pair struct {
    range1 SectorRange;
    range2 SectorRange;
}

func numberOfOverlappingPairs(filename string) int {
    spew.Dump(sectorRanges(filename))
    return 0
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
