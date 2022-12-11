package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestPart1(t *testing.T) {
    t.Run("returns number of fully overlapping pairs", func (t *testing.T) {
        require.Equal(t, 2, numberOFullyOverlappingPairs("data/test_input"))
    })
}
func TestPart2(t *testing.T) {
    t.Run("returns number pairs that overlap at all", func (t *testing.T) {
        require.Equal(t, 4, numberOfOverlappingPairs("data/test_input"))
    })
}
