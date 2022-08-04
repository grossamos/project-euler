package solver

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestGetNextTriangleNumber(t *testing.T) {
    tg := NewTriangeGenerator()
    assert.Equal(t, tg.Next(), uint(1))
    assert.Equal(t, tg.Next(), uint(3))
    assert.Equal(t, tg.Next(), uint(6))
    assert.Equal(t, tg.Next(), uint(10))
    assert.Equal(t, tg.Next(), uint(15))
    assert.Equal(t, tg.Next(), uint(21))
    assert.Equal(t, tg.Next(), uint(28))
    assert.Equal(t, tg.Next(), uint(36))
    assert.Equal(t, tg.Next(), uint(45))
    assert.Equal(t, tg.Next(), uint(55))
    assert.Equal(t, tg.Next(), uint(66))
}

func TestGetNumOfDivisors(t *testing.T) {
    assert.Equal(t, getNumberOfDivisors(28), uint(6))
    assert.Equal(t, getNumberOfDivisors(6), uint(4))
    assert.Equal(t, getNumberOfDivisors(10), uint(4))
}

func TestGetTriangleWithNumOfDivisors(t *testing.T) {
    assert.Equal(t, GetTriangeWithNumOfDivisors(5), uint(28))
    assert.Equal(t, GetTriangeWithNumOfDivisors(3), uint(6))
}
