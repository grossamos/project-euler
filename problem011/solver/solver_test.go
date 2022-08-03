package solver

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

var array = [][]uint {
    {  8, 22, 34, 24},
    { 16, 17, 45, 21},
    { 34, 12, 75,  1},
    { 26, 16, 61, 11},
}

func TestGetAdjacentProducts(t *testing.T) {
    got, err := getAdjacentProducts(array, 0, 0, 4)

    assert.Nil(t, err)
    assert.Contains(t, got, uint(8 * 16 * 34 * 26))
    assert.Contains(t, got, uint(8 * 17 * 75 * 11))
    assert.Contains(t, got, uint(8 * 22 * 34 * 24))

    got, err = getAdjacentProducts(array, 3, 3, 4)
    assert.Nil(t, err)
    assert.Contains(t, got, uint(24 * 21 * 1 * 11))
    assert.Contains(t, got, uint(8 * 17 * 75 * 11))
}

func TestGetGreatestNumFromArray(t *testing.T) {
    got := GetGreatestProductFromArray(array, 4)
    assert.Equal(t, got, uint(34 * 45 * 75 * 61))
}
