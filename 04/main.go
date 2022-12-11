package main

import "io/ioutil"

func numberOfOverlappingPairs(filename string) int {
    readInput(filename)
    return 0
}

func readInput(filename string) string {
	data, err := ioutil.ReadFile(filename)

	if err != nil {
		panic(err)
	}

    return string(data)
}
