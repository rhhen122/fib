package main

import "fmt"

func main() {
    a := 0
    b := 1
    o := 0
    for {
    	fmt.Println(a)
    	o = a + b
    	a = b
    	b = o
    }
}