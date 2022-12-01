package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestPart1(t *testing.T) {
    require.Equal(t, 24000, determineElfWithMostCaloriesFromFile("data/test.input"))
}
