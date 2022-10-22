## Sliding Puzzle

The code should be able to solve a random sliding puzzle.

### Requirements
[The game](https://appzaza.com/tile-slide-game) starts with a board consisting of blocks numbered 1 through N
and one blank block represented by the number 0. The goal is to arrange
the tiles according to their numbers. Moving is done by moving the blocks
on top, bottom, left and right in place of the empty block.

At the input is given the number N - the number of blocks with numbers
(8, 15, 24, etc.), the number I - the index of the position of zero (the empty
block) in the decision (using -1 the default zero index position is set at the
bottom right) and then the layout of the board is introduced. Using the A*
(or IDA*) algorithm and the Manhattan distance heuristics (or Hemming
distance), derive:

- In the first line, the length of the "optimal" path from start to destination.
- The appropriate steps (in a new line for each one) that are taken to reach
the final state. The steps are left, right, up and down


**Keep in mind that not every puzzle is solvable.** [You can check whether the
puzzle is solvable or directly use valid examples.](https://www.cs.princeton.edu/courses/archive/spring18/cos226/assignments/8puzzle/index.html)


### Sample test case
Input:
```
8
-1
1 2 3
4 5 6
0 7 8
```

Output:
```
2
left
left
```
