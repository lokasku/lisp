(cond ((> 1 2) 1)
      (t (o-k "ok" 2)))

(macro foo (u o)
  (print "hel")
  (print "lo"))

(define bar (x)
  x)

(var *egg* (- +10 -10))

(\ (x) (+ (* 2 x) 1))