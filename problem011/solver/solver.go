package solver

import "errors"

func getAdjacentProducts(array [][]uint, x uint, y uint, length uint) ([]uint, error) {
    products := []uint{}
    if len(array) == 0 {
        return products, errors.New("empty array")
    }

    rightOkay := len(array[0]) >= int(x + length)
    leftOkay := int(x) - int(length) > 0
    downOkay := len(array) >= int(y + length)
    upOkay := int(y) - int(length) > 0 

    // right
    if rightOkay {
        product := uint(1)
        for index := uint(0); index < length; index++ {
            product *= array[y][x + index]
        }
        products = append(products, product)
    }

    //left
    if leftOkay {
        product := uint(1)
        for index := uint(0); index < length; index++ {
            product *= array[y][x - index]
        }
        products = append(products, product)
    }

    // down
    if downOkay {
        product := uint(1)
        for index := uint(0); index < length; index++ {
            product *= array[y + index][x]
        }
        products = append(products, product)
    }

    // up
    if upOkay {
        product := uint(1)
        for index := uint(0); index < length; index++ {
            product *= array[y - index][x]
        }
        products = append(products, product)
    }

    // diagonal down-right
    if downOkay && rightOkay {
    }

    // diagonal down-left
    // diagonal up-right
    // diagonal up-left

    

    return products, nil
}

func generateProductFromArray(x uint, y uint, down bool, right bool, length uint, array uint) uint {
    // TODO use this function to replace repetition in top
    product := uint(1)
    for index := uint(0); index < length; index++ {
        product *= array[y + index][x + index]
    }
    return product
}
