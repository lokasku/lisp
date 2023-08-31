use crate::parser::parser::{
    Sexp,
    SexpT,
    independant_sexp
};
use crate::errors::{
    Error,
    EvalError
};


pub fn print(sexp: Result<Sexp, Error>) {
    match sexp {
        Ok(s) => match s.sexpt {
            SexpT::Atom(a) => print!("{}", a),
            SexpT::List(v) => {
                print!("(");
                for s in v {
                    print(Ok(s));
                }
                print!(") ");
            }
        }
        Err(e) => println!("{}", e)
    }
}

pub fn quote(sexp: Sexp) -> Sexp { sexp }

pub fn car(list: Sexp) -> Result<Sexp, Error> {
    if let SexpT::List(v) = list.sexpt {
        match v.get(0) {
            Some(sexp) => Ok(sexp.clone()),
            None => Ok(independant_sexp(SexpT::List(Vec::new())))
        }
    } else {
        Err(Error::EvalError(EvalError::TypeMismatch(list.sexpt, String::from("list"), list.pos)))
    }
}

pub fn cdr(list: Sexp) -> Result<Sexp, Error> {
    if let SexpT::List(v) = list.sexpt {
        match v.get(1..) {
            Some(v) => Ok(independant_sexp(SexpT::List(v.to_vec()))),
            None => Ok(independant_sexp(SexpT::List(Vec::new())))
        }
    } else {
        Err(Error::EvalError(EvalError::TypeMismatch(list.sexpt, String::from("list"), list.pos)))
    }
}

pub fn cons(item: Sexp, list: Sexp) -> Result<Sexp, Error> {
    if let SexpT::List(mut v) = list.sexpt {
        v.insert(0, item);
        Ok(independant_sexp(SexpT::List(v)))
    } else {
        Err(Error::EvalError(EvalError::TypeMismatch(list.sexpt, String::from("list"), list.pos)))
    }
}
