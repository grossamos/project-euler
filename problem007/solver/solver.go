package solver

type PrimeGenerator struct {
    known_primes []uint
    counter uint
}

func NewPrimeGenerator() PrimeGenerator {
    return PrimeGenerator{[]uint{2}, 2}
}

func (pg *PrimeGenerator) next() uint {

    for true {
        pg.counter++
        isPrime := true
        for _, prime := range pg.known_primes {
            if pg.counter % prime == 0 {
                isPrime = false
                break
            }
        }
        if isPrime {
            pg.known_primes = append(pg.known_primes, pg.counter)
            return pg.counter
        }
    }
    // this code should never be reached
    panic(1)
}

func GetNthPrime(n uint) uint {
    n--
    primeGen := NewPrimeGenerator()
    for i := uint(0); i < n; i++ {
        primeGen.next()
    }
    return primeGen.counter
}
