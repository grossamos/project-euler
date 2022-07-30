package solver

import "fmt"

type PrimeGenerator struct {
    knownPrimes []uint
    counter uint
}

func NewPrimeGenerator() PrimeGenerator {
    return PrimeGenerator{[]uint{2}, 2}
}

func (pg *PrimeGenerator) next() uint {
    for {
        pg.counter++
        isPrime := true
        for _, prime := range pg.knownPrimes {
            if pg.counter % prime == 0 {
                isPrime = false
                break
            }
        }
        if isPrime {
            pg.knownPrimes = append(pg.knownPrimes, pg.counter)
            return pg.counter
        }
    }
}

func GetSumOfPrimes(limit uint) uint {
    sum := uint(2)
    pg := NewPrimeGenerator()
    nextPrime := pg.next()
    for nextPrime <= limit {
        fmt.Println(nextPrime)
        sum += nextPrime
        nextPrime = pg.next()
    }
    return sum
}
