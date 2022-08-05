package solver

import (
	"fmt"
	"math"
	"strconv"
)

func getNumParts(bigNum string) (uint64, uint64, uint64) {
    p1, _ := strconv.ParseUint(bigNum[0:14], 10, 64)
    p2, _ := strconv.ParseUint(bigNum[14:32], 10, 64)
    p3, _ := strconv.ParseUint(bigNum[32:50], 10, 64)
    return p1, p2, p3
}

func addNumParts(a1 uint64, a2 uint64, a3 uint64, b1 uint64, b2 uint64, b3 uint64) (uint64, uint64, uint64) {
    firstDigit := a3 / uint64(math.Pow10(17)) + b3 / uint64(math.Pow10(17))
    fmt.Println(firstDigit)
    c3 := a3 + b3
    if firstDigit != a3 / uint64(math.Pow10(17)) {
        a2 += 1
    }
    firstDigit = a2 / uint64(math.Pow10(17)) + b2 / uint64(math.Pow10(17))
    fmt.Println(firstDigit)
    c2 := a2 + b2
    if firstDigit != a3 / uint64(math.Pow10(17)) {
        a1 += 1
    }
    c1 := a1 + b1
    return c1, c2, c3
}
