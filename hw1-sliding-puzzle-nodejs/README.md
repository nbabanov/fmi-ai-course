## Sliding Puzzle

The code should be able to solve a random sliding puzzle.

Prerequisites: NodeJS 16+.

Run: `npm start` in the folder to start the program.

The implementation is in `main.ts`.

### Requirements
[The game](https://appzaza.com/tile-slide-game) starts with a board consisting of blocks numbered 1 through N
and one blank block represented by the number 0. The goal is to arrange
the tiles according to their numbers. Moving is done by moving the blocks
on top, bottom, left and right in place of the empty block.

At the input is given the number N - the number of blocks with numbers
(8, 15, 24, etc.), the number I - the index of the position of zero (the empty
block) in the decision (using -1 the default zero index position is set at the
bottom right) and then the layout of the board is introduced. 

Using the IDA* algorithm and the Manhattan distance heuristics, derive:
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


### IDA* pseudocode:

```ts
path              current search path (acts like a stack)
node              current node (last node in current path)
g                 the cost to reach current node
f                 estimated cost of the cheapest path (root..node..goal)
h(node)           estimated cost of the cheapest path (node..goal)
cost(node, succ)  step cost function
is_goal(node)     goal test
successors(node)  node expanding function, expand nodes ordered by g + h(node)
ida_star(root)    return either NOT_FOUND or a pair with the best path and its cost
 
procedure ida_star(root)
    bound := h(root)
    path := [root]
    loop
        t := search(path, 0, bound)
        if t = FOUND then return (path, bound)
        if t = ∞ then return NOT_FOUND
        bound := t
    end loop
end procedure

function search(path, g, bound)
    node := path.last
    f := g + h(node)
    if f > bound then return f
    if is_goal(node) then return FOUND
    min := ∞
    for succ in successors(node) do
        if succ not in path then
            path.push(succ)
            t := search(path, g + cost(node, succ), bound)
            if t = FOUND then return FOUND
            if t < min then min := t
            path.pop()
        end if
    end for
    return min
end function
```
