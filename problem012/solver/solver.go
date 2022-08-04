package solver

type TriangeGenerator struct {
    currentNum uint
    currentCounter uint
}

func NewTriangeGenerator() TriangeGenerator {
    return TriangeGenerator{0, 0}
}

func (tg *TriangeGenerator) Next() uint {
    tg.currentCounter += 1
    tg.currentNum += tg.currentCounter
    return tg.currentNum
}

func getNumberOfDivisors(base uint) uint {
    // a number is allways divisible by 1 and itself
    counter := uint(2)
    for i := uint(2); i <= base / 2; i++ {
        if base % i == 0 {
            counter++
        }
    }
    return counter
}

func GetTriangeWithNumOfDivisors(numOfDiv uint) uint {
    tg := NewTriangeGenerator()
    divNum := uint(0)
    for divNum <= numOfDiv {
        tg.Next()
        divNum = getNumberOfDivisors(tg.currentNum)
    }
    return tg.currentNum
}
