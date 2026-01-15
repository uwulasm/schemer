# schemer
this is a simple custom scheme variant implementation

# example
```scheme
λ (= (+ 2 2) 4)
true
λ (if (= 2 3) (display "yay!") (display "nay!"))
nay!
λ (define (double x) (+ x x))
λ (double 16)
32
λ (define (sum n) (if (= n 0) 0 (+ n (sum (- n 1)))))
λ (sum 100)
5050
```
