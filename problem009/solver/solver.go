package solver

import "errors"

func isPythagoreanTripple(a, b, c uint) bool {
    return a * a + b * b == c * c
}

func FindPythagoreanTripple(sum uint) (uint, uint, uint, error) {
    for a := uint(1); a < sum; a++ {
        for b := a; b < sum; b++ {
            if isPythagoreanTripple(a, b, sum - a - b) {
                return a, b, sum - a - b, nil
            }
        }
    }
    return 0, 0, 0, errors.New("No tripple found")
}
