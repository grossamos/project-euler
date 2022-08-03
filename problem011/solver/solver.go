package solver

import (
	"errors"
)

func getAdjacentProducts(array [][]uint, x uint, y uint, length uint) ([]uint, error) {
    products := []uint{}
    if len(array) == 0 {
        return products, errors.New("empty array")
    }

    rightOkay := len(array[0]) >= int(x + length)
    leftOkay := int(x + 1) - int(length) >= 0
    downOkay := len(array) >= int(y + length)
    upOkay := int(y + 1) - int(length) >= 0 

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
        product := uint(1)
        for index := uint(0); index < length; index++ {
            product *= array[y + index][x + index]
        }
        products = append(products, product)
    }

    // diagonal down-left
    if downOkay && leftOkay {
        product := uint(1)
        for index := uint(0); index < length; index++ {
            product *= array[y + index][x - index]
        }
        products = append(products, product)
    }

    // diagonal up-right
    if upOkay && rightOkay {
        product := uint(1)
        for index := uint(0); index < length; index++ {
            product *= array[y - index][x + index]
        }
        products = append(products, product)
    }

    // diagonal up-left
    if upOkay && leftOkay {
        product := uint(1)
        for index := uint(0); index < length; index++ {
            product *= array[y - index][x - index]
        }
        products = append(products, product)
    }


    return products, nil
}

func GetGreatestProductFromArray(array [][]uint, limit uint) uint {
    greatestProduct := uint(0)
    for i := uint(0); i < uint(len(array)); i++ {
        for j := uint(0); j < uint(len(array[0])); j++ {
            products, _ := getAdjacentProducts(array, j, i, limit)
            for _, product := range products {
                if product > greatestProduct {
                    greatestProduct = product
                }
            }
        }
    }
    return greatestProduct
}
