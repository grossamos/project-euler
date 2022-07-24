package calc

import (
    "testing"
)

type testCase struct {
    arg1 uint
    want uint
}

func TestSumOfSqares(t *testing.T) {
    testCases := []testCase {
        {10, 385},
        {11, 506},
    }
    for _, tc := range testCases {
        got := SumOfSquares(tc.arg1)
        if got != tc.want {
            t.Errorf("Expeced %d, but got %d", tc.want, got)
        }
    }
}

func TestSquarOfSum(t *testing.T) {
    testCases := []testCase {
        {10, 3025},
        {11, 4356},
    }
    for _, tc := range testCases {
        got := SquareOfSums(tc.arg1)
        if got != tc.want {
            t.Errorf("Expeced %d, but got %d", tc.want, got)
        }
    }
}

