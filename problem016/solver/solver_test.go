package solver

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func CanDoubleString(t *testing.T) {
    str1 := "2"
    assert.Equal(str1, DoubleString("4"))
}
