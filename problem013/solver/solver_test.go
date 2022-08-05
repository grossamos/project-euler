package solver

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestGenerateNumParts(t *testing.T) {
    p1, p2, p3 := getNumParts("37107287533902102798797998220837590246510135740250")
    assert.Equal(t, p1, uint64(37107287533902))
    assert.Equal(t, p2, uint64(102798797998220837))
    assert.Equal(t, p3, uint64(590246510135740250))
}

func TestAddNumParts(t *testing.T) {
    t1, t2, t3 := getNumParts("37107287533902102798797998220837590246510135740250")
    n1, n2, n3 := getNumParts("46376937677490009712648124896970078050417018260538")
    p1, p2, p3 := addNumParts(t1, t2, t3, n1, n2, n3)

    assert.Equal(t, fmt.Sprintf("%d%d%d", p1, p2, p3), "83484225211392112511446123117807668296927154000788")
}
