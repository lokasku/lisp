(car (cons (quote a)
      (cons (eval (quote (atom
        (cond (() 1)
          (t 2)
          (t 3))))) ())))
