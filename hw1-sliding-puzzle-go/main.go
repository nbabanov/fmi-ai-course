package main

import (
	"fmt"
	"math"
	"reflect"

	slices "golang.org/x/exp/slices"
)

type Point2D struct {
	X int
	Y int
}

func getManhattenDistance(a Point2D, b Point2D) int {
	return int(math.Abs(float64(a.X)-float64(b.X))) + int(math.Abs(float64(a.Y)-float64(b.Y)))
}

type Node struct {
	cost       int
	board      []int
	pathToNode []string
}
type Heuristic func(root *Node) int
type GoalPredicate func(node *Node) bool

type SearchResult struct {
	cost int
	node *Node
}

type BoardMove int

const (
	Top BoardMove = iota
	Right
	Bottom
	Left
)

var BOARD_MOVE = struct {
	Top    BoardMove
	Right  BoardMove
	Bottom BoardMove
	Left   BoardMove
}{Top, Right, Bottom, Left}

func boardMoveToString(move BoardMove) string {
	switch move {
	case 0:
		return "top"
	case 1:
		return "right"
	case 2:
		return "bottom"
	case 3:
		return "left"
	}

	return ""
}

func getMoveIndex(boardSide int, currentIndex int, move BoardMove) int {
	switch move {
	case BOARD_MOVE.Top:
		return currentIndex - boardSide
	case BOARD_MOVE.Bottom:
		return currentIndex + boardSide
	case BOARD_MOVE.Left:
		if currentIndex%boardSide == 0 {
			return -1
		}
		return currentIndex - 1
	case BOARD_MOVE.Right:
		if currentIndex%boardSide == boardSide-1 {
			return boardSide * boardSide
		}

		return currentIndex + 1
	}

	return currentIndex
}

func indexOf(array *[]int, value int) int {
	var arrayLength = len(*array)
	for i := 0; i < arrayLength; i++ {
		if (*array)[i] == 0 {
			return i
		}
	}

	return -1
}

func getZeroIndex(array *[]int) int {
	return indexOf(array, 0)
}

func getSuccessors(boardSide int, node Node) []Node {
	fmt.Println("Node:", node.board)
	var zeroIndex = getZeroIndex(&node.board)
	fmt.Println("Zero index:", zeroIndex)
	var boardLength = len(node.board)

	var result []Node

	for i := 0; i < 4; i++ {
		var moveIndex = getMoveIndex(boardSide, zeroIndex, BoardMove(i))

		fmt.Println("Move index: ", moveIndex)

		if moveIndex >= 0 && moveIndex < boardLength {
			fmt.Println("Valid Move index: ", moveIndex)
			var board = make([]int, boardLength)
			copy(board, node.board)
			var path []string
			copy(path, node.pathToNode)
			var boardMoveString = boardMoveToString(BoardMove(i))
			path = append(path, boardMoveString)
			board[zeroIndex] = board[moveIndex]
			board[moveIndex] = 0
			result = append(result, Node{
				0, // TODO: Use manhatten to calculate cost!
				board,
				path,
			})

			fmt.Println("RES: ", result)
		}
	}

	fmt.Println("Successors: ", result)

	return result
}

type StepCost func(a *Node, b *Node) int

func Map[T, U any](ts []T, f func(T) U) []U {
    us := make([]U, len(ts))
    for i := range ts {
        us[i] = f(ts[i])
    }
    return us
}

func search(boardSide int, heuristic Heuristic, stepCost StepCost, isGoal GoalPredicate, path *[]*Node, currentCost int, threshold int) SearchResult {
	var node = (*path)[len(*path)-1]
	var cheapestPathCost = currentCost + heuristic(node)

	if cheapestPathCost > threshold {
		return SearchResult{cheapestPathCost, nil}
	}

	if isGoal(node) {
		return SearchResult{cheapestPathCost, node}
	}

	var min = math.MaxInt

	var successors = getSuccessors(boardSide, *node)

	for i := 0; i < len(successors); i++ {
		var successor = successors[i]

		var isInPath = slices.IndexFunc(*path, func(element *Node) bool {
			return reflect.DeepEqual((*element).board, successor.board)
		}) != -1

		fmt.Println("Search current path: ", Map(*path, func (node *Node) Node {
			return *node;
		}))
		fmt.Println("Search successor: ", successor)
		fmt.Println("is in path: ", isInPath)

		if !isInPath {
			*path = append(*path, &successor)
			var searchResult = search(boardSide, heuristic, stepCost, isGoal, path, currentCost+stepCost(node, &successor), threshold)

			if searchResult.node != nil {
				return searchResult
			}

			if searchResult.cost < min {
				min = searchResult.cost
			}

			*path = (*path)[:len(*path)-1]
		}
	}

	return SearchResult{min, nil}
}

type IDAResult struct {
	path      []string
	threshold int
}

func idaStar(boardSide int, heuristic Heuristic, stepCost StepCost, isGoal GoalPredicate, root *Node) *IDAResult {
	var threshold = heuristic(root)
	var path = []*Node{root}

	for {
		var result = search(boardSide, heuristic, stepCost, isGoal, &path, 0, threshold)
		if isGoal(result.node) {
			fmt.Println("RESULT NODE:", result.node.board)
			return &IDAResult{result.node.pathToNode, threshold}
		}

		if slices.Contains(path, result.node) {
			return nil
		}

		threshold = result.cost
	}
}

func isGoalCreator(goalIndex int) func(node *Node) bool {
	isGoal := func(node *Node) bool {
		if node == nil {
			return false
		}

		return getZeroIndex(&((*node).board)) == goalIndex
	}

	return isGoal
}

func boardSizeToBoardSide(size int) int {
	return int(math.Abs(math.Sqrt(float64(size + 1))))
}

func index1DToIndex2D(boardSide int, index int) Point2D {
	var x int = index % boardSide
	var y int = index / boardSide

	return Point2D{x, y}
}

func boardToPoint2D(boardSide int, board *[]int) Point2D {
	return index1DToIndex2D(boardSide, getZeroIndex(board))
}

func main() {
	var boardSize int = 8
	var boardSide = boardSizeToBoardSide(boardSize)
	var isGoal = isGoalCreator(8)
	var board = []int{1, 2, 3, 4, 5, 6, 0, 7, 8}
	var heuristic = func(node *Node) int {
		return getManhattenDistance(boardToPoint2D(boardSide, &board), index1DToIndex2D(boardSide, boardSize))
	}

	var stepCost StepCost = func(a *Node, b *Node) int {
		return getManhattenDistance(boardToPoint2D(boardSide, &(*a).board), boardToPoint2D(boardSide, &(*b).board))
	}

	var root = &Node{0, board, []string{}}

	var result = idaStar(boardSide, heuristic, stepCost, isGoal, root)

	fmt.Println("Threshold: ", (*result).threshold)
	fmt.Printf("Path: %s", (*result).path)
}
