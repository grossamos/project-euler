package solver

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestIsPythagoreanTripple(t *testing.T) {
    assert.True(t, isPythagoreanTripple(3, 4, 5))
}

func FindsPythagoreanTrippleForNum(t *testing.T) {
    a, b, c, err := FindPythagoreanTripple(5)
    assert.NotEqual(t, err, nil)
    assert.Equal(t, a, 3)
    assert.Equal(t, b, 4)
    assert.Equal(t, c, 5)
}

