package solver

import "testing"

func TestGeneratePrimes(t *testing.T) {
    primes := []uint{3, 5, 7, 11, 13, 17, 19}
    primeGenerator := NewPrimeGenerator()
    for _, prime := range primes {
        got := primeGenerator.next()
        if got != prime {
            t.Errorf("Expected %d, but got %d", prime, got)
        }
    }
}

type testCase struct {
    arg1 uint
    want uint
}

func TestSumOfPrimes(t *testing.T) {
    testCases := []testCase {
        {4, 5},
        {6, 10},
        {12, 28},
        {15, 41},
    }
    for _, tc := range testCases {
        got := GetSumOfPrimes(tc.arg1)
        if tc.want != got {
            t.Errorf("Expected %d, but got %d", tc.want, got)
        }
    }
}
