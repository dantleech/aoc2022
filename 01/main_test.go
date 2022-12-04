package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestPart1_returnsNumberOfCaloriesCariedByElfCarryingTheMostCalories(t *testing.T) {
    require.Equal(t, 24000, determineNumberOfCaloriesCarriedByElfWithMostCalories("data/test.input"))
}

func TestPart2_numberOfCaloriesBeingCarriedByTheTop3ElvesCarryingTheMostCallories(t *testing.T) {
    require.Equal(t, 45000, determineNumberOfCaloriesCarriedByTopThreeElvesWithMostCalories("data/test.input"))
}
