```
algorithm MIN-CONFLICTS is
    input: console.csp, A constraint satisfaction problem.
           max_steps, The number of steps allowed before giving up.
           current_state, An initial assignment of values for the variables in the csp.
    output: A solution set of values for the variable or failure.

    for i ← 1 to max_steps do
        if current_state is a solution of csp then
            return current_state
        set var ← a randomly chosen variable from the set of conflicted variables CONFLICTED[csp]
        set value ← the value v for var that minimizes CONFLICTS(var,v,current_state,csp)
        set var ← value in current_state

    return failure
```
