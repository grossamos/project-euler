package main

import (
	"fmt"
	"os"
	"problem009/solver"
)

func main() {
    a, b, c, err := solver.FindPythagoreanTripple(1000);
    if err != nil {
        fmt.Println(err.Error())
        os.Exit(1)
    }
    fmt.Println(a * b * c)
}
