package main

import "fmt"

func main() {
	a := 0
	b := 1
	for {
		a += b
		fmt.Println(a)
		b += a
		fmt.Println(b)
	}
}
