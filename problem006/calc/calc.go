package calc

func SumOfSquares(limit uint) (sum uint) {
    for index := uint(1); index <= limit; index++ {
        sum += index * index
    }
    return
}

func SquareOfSums(limit uint) uint {
    square := uint(0)
    for index := uint(1); index <= limit; index++ {
        square += index
    }
    return square * square
}
