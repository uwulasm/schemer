(display "welcome to schemer!")

(define (factorial n)
  (if (= n 0)
    1
    (* n (factorial (- n 1)))))


(define (iterate i f)
  (if (= i 0)
    ()
    (begin (f i)
           (iterate (- i 1)))))
