(define add (x y)
     (+ x y))

(macro double (x)
       '(* 2 ,x))

(var foo "hello, world")
