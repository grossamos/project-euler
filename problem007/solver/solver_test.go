package solver

import "testing"

type testCase struct {
    arg1 uint
    want bool
}

func TestGenerateNextPrim(t *testing.T) {
    testCases := []uint{3, 5, 7, 11, 13, 17, 19, 23}

    primes := NewPrimeGenerator()
    for i := 0; i < len(testCases); i++ {
        got := primes.next()
        if got != testCases[i] {
            t.Errorf("Expected %d, recieved %d", testCases[i], got)
        }
    }
}

func TestGenerateNthPrime(t * testing.T) {
    got := GetNthPrime(6)
    if got != 13 {
        t.Errorf("Expeced %d, recieved %d", 13, got)
    }

    got = GetNthPrime(9)
    if got != 23 {
        t.Errorf("Expeced %d, recieved %d", 6, got)
    }
}
