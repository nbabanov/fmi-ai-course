package main

import "math"
import "fmt"

type Point2D struct {
	X int
	Y int
}

func getManhattenDistance(a Point2D, b Point2D) int {
	return int(math.Abs(float64(a.X) - float64(b.X))) + int(math.Abs(float64(a.Y) - float64(b.Y)))
}

func main() {
	fmt.Println(getManhattenDistance(Point2D{1, 1}, Point2D{2, 2}))
}
