#![feature(box_syntax, box_patterns)]

extern crate nom;
use nom::{IResult};

use std::fmt;
use Noun::*;

fn main() {
    println!(
        "{}",
        Cell(Box::new((Atom(12), Cell(Box::new((Atom(487), Atom(325)))))))
    );
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Noun {
    Atom(u64),
    Cell(Box<(Noun, Noun)>),
}

impl fmt::Display for Noun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            Atom(a) => format!("{}", a),
            Cell(box (n1, n2)) => format!("[{}  {}]", n1, n2),
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    Noun(Noun),
    Wut(Noun),
    Lus(Noun),
    Tis(Noun),
    Net(Noun),
    Hax(Noun),
    Tar(Noun),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Error(Noun);

pub type Possibly<T> = Result<T, Error>;

pub fn eval(expr: Expr) -> Possibly<Noun> {
    match expr {
        Expr::Noun(n) => Ok(n),
        Expr::Wut(n) => wut(n),
        Expr::Lus(n) => lus(n),
        Expr::Tis(n) => tis(n),
        Expr::Net(n) => net(n),
        Expr::Hax(n) => hax(n),
        Expr::Tar(n) => tar(n),
    }
}



// `?`
// ?[a b]              0
// ?a                  1
pub fn wut(input: Noun) -> Possibly<Noun> {
    match input {
        Cell(_) => Ok(Atom(0)),
        Atom(_) => Ok(Atom(1)),
    }
}

// `=`
// =[a a]              0
// =[a b]              1
pub fn tis(input: Noun) -> Possibly<Noun> {
    match input {
        Atom(_) => Err(Error(input)),
        Cell(box (n1, n2)) => {
            if n1 == n2 {
                return Ok(Atom(0));
            } else {
                return Ok(Atom(1));
            }
        }
    }
}

// `+`
// +[a b]              +[a b]
// +a                  1 + a
pub fn lus(input: Noun) -> Possibly<Noun> {
    match input {
        Atom(n) => Ok(Atom(n + 1)),
        other => Err(Error(other)),
    }
}

// `/`
// /[1 a]              a
// /[2 a b]            a
// /[3 a b]            b
// /[(a + a) b]        /[2 /[a b]]
// /[(a + a + 1) b]    /[3 /[a b]]
// /a                  /a
pub fn net(input: Noun) -> Possibly<Noun> {
    match input {
        Cell(box (a, Cell(box (b, c)))) => match a {
            Atom(i) => match i {
                1 => Ok(Cell(Box::new((b, c)))),
                2 => Ok(b),
                3 => Ok(c),
                i if i > 3 => {
                    let inner_net = net(Cell(Box::new((Atom(i / 2), Cell(Box::new((b, c)))))))?;
                    net(Cell(Box::new((Atom(2 + (i % 2)), inner_net))))
                }
                _ => Err(Error(a)),
            },
            other => Err(Error(other)),
        },
        other => Err(Error(other)),
    }
}

// `#`
// #[1 a b]            a
// #[(a + a) b c]      #[a [b /[(a + a + 1) c]] c]
// #[(a + a + 1) b c]  #[a [/[(a + a) c] b] c]
// #a                  #a
pub fn hax(input: Noun) -> Possibly<Noun> {
    match input {
        Atom(_) => Err(Error(input)),
        Cell(box (a, Cell(box (b, c)))) => match a {
            Cell(_) => Err(Error(a)),
            Atom(i) => match i {
                1 => Ok(b),
                n if n % 2 == 0 => {
                    let c_copy = c.clone();
                    let inner_net = Cell(Box::new((b, net(Cell(Box::new((Atom(n + 1), c))))?)));
                    hax(Cell(Box::new((
                        Atom(n / 2),
                        Cell(Box::new((inner_net, c_copy))),
                    ))))
                }
                n if n % 2 == 1 => {
                    let c_copy = c.clone();
                    let inner_net = Cell(Box::new((net(Cell(Box::new((Atom(n - 1), c))))?, b)));
                    hax(Cell(Box::new((
                        Atom((n - 1) / 2),
                        Cell(Box::new((inner_net, c_copy))),
                    ))))
                }
                _ => Err(Error(a)),
            },
        },
        other => Err(Error(other)),
    }
}

// `*`
// *[a [b c] d]        [*[a b c] *[a d]]

// *[a 0 b]            /[b a]
// *[a 1 b]            b
// *[a 2 b c]          *[*[a b] *[a c]]
// *[a 3 b]            ?*[a b]
// *[a 4 b]            +*[a b]
// *[a 5 b c]          =[*[a b] *[a c]]

// *[a 6 b c d]        *[a *[[c d] 0 *[[2 3] 0 *[a 4 4 b]]]]
// *[a 7 b c]          *[*[a b] c]
// *[a 8 b c]          *[[*[a b] a] c]
// *[a 9 b c]          *[*[a c] 2 [0 1] 0 b]
// *[a 10 [b c] d]     #[b *[a c] *[a d]]

// *[a 11 [b c] d]     *[[*[a c] *[a d]] 0 3]
// *[a 11 b c]         *[a c]

// *a                  *a
pub fn tar(input: Noun) -> Possibly<Noun> {
    match input {
        Atom(_) => Err(Error(input)),

        Cell(box (a, Cell(box (Cell(box (b, c)), d)))) => Ok(Cell(Box::new((
            tar(Cell(Box::new((a.clone(), Cell(Box::new((b, c)))))))?,
            tar(Cell(Box::new((a, d))))?,
        )))),

        Cell(box (a, Cell(box (Atom(0), b)))) => net(Cell(Box::new((b, a)))),

        Cell(box (_a, Cell(box (Atom(1), b)))) => Ok(b),

        Cell(box (a, Cell(box (Atom(2), Cell(box (b, c)))))) => tar(Cell(Box::new((
            tar(Cell(Box::new((a.clone(), b))))?,
            tar(Cell(Box::new((a, c))))?,
        )))),

        Cell(box (a, Cell(box (Atom(3), b)))) => wut(tar(Cell(Box::new((a, b))))?),

        Cell(box (a, Cell(box (Atom(4), b)))) => lus(tar(Cell(Box::new((a, b))))?),

        Cell(box (a, Cell(box (Atom(5), Cell(box (b, c)))))) => tis(Cell(Box::new((
            tar(Cell(Box::new((a.clone(), b))))?,
            tar(Cell(Box::new((a, c))))?,
        )))),

        Cell(box (a, Cell(box (Atom(6), Cell(box (b, Cell(box (c, d)))))))) => {
            tar(Cell(Box::new((
                a.clone(),
                tar(Cell(Box::new((
                    (Cell(Box::new((c, d)))),
                    Cell(Box::new((
                        Atom(0),
                        tar(Cell(Box::new((
                            Cell(Box::new((Atom(2), Atom(3)))),
                            Cell(Box::new((
                                Atom(0),
                                tar(Cell(Box::new((
                                    a,
                                    Cell(Box::new((Atom(4), Cell(Box::new((Atom(4), b)))))),
                                ))))?,
                            ))),
                        ))))?,
                    ))),
                ))))?,
            ))))
        }

        Cell(box (a, Cell(box (Atom(7), Cell(box (b, c)))))) => {
            tar(Cell(Box::new((tar(Cell(Box::new((a, b))))?, c))))
        }

        Cell(box (a, Cell(box (Atom(8), Cell(box (b, c)))))) => tar(Cell(Box::new((
            Cell(Box::new((tar(Cell(Box::new((a.clone(), b))))?, a))),
            c,
        )))),

        Cell(box (a, Cell(box (Atom(9), Cell(box (b, c)))))) => tar(Cell(Box::new((
            tar(Cell(Box::new((a, c))))?,
            Cell(Box::new((
                Atom(2),
                Cell(Box::new((
                    Cell(Box::new((Atom(0), Atom(1)))),
                    Cell(Box::new((Atom(0), b))),
                ))),
            ))),
        )))),

        Cell(box (a, Cell(box (Atom(10), Cell(box (Cell(box (b, c)), d)))))) => {
            hax(Cell(Box::new((
                b,
                Cell(Box::new((
                    tar(Cell(Box::new((a.clone(), c))))?,
                    tar(Cell(Box::new((a, d))))?,
                ))),
            ))))
        }

        Cell(box (a, Cell(box (Atom(11), Cell(box (Cell(box (_b, c)), d)))))) => {
            tar(Cell(Box::new((
                Cell(Box::new((
                    tar(Cell(Box::new((a.clone(), c))))?,
                    tar(Cell(Box::new((a, d))))?,
                ))),
                Cell(Box::new((Atom(0), Atom(3)))),
            ))))
        }

        Cell(box (a, Cell(box (Atom(11), Cell(box (_b, c)))))) => tar(Cell(Box::new((a, c)))),

        other => Err(Error(other)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wut() {
        assert_eq!(wut(Cell(Box::new((Atom(1), Atom(2))))), Ok(Atom(0)));
        assert_eq!(wut(Atom(5)), Ok(Atom(1)));
    }

    #[test]
    fn test_tis() {
        assert_eq!(tis(Cell(Box::new((Atom(1), Atom(1))))), Ok(Atom(0)));
        assert_eq!(tis(Cell(Box::new((Atom(1), Atom(2))))), Ok(Atom(1)));
        assert_eq!(tis(Atom(5)), Err(Error(Atom(5))))
    }

    #[test]
    fn test_lus() {
        assert_eq!(lus(Atom(5)), Ok(Atom(6)));
        assert_eq!(
            lus(Cell(Box::new((Atom(0), Atom(1))))),
            Err(Error(Cell(Box::new((Atom(0), Atom(1))))))
        )
    }

    #[test]
    fn test_net() {
        assert_eq!(
            net(Cell(Box::new((
                Atom(1),
                Cell(Box::new((Atom(531), Cell(Box::new((Atom(25), Atom(99)))))))
            )))),
            Ok(Cell(Box::new((
                Atom(531),
                Cell(Box::new((Atom(25), Atom(99))))
            ))))
        );
        assert_eq!(
            net(Cell(Box::new((
                Atom(6),
                Cell(Box::new((Atom(531), Cell(Box::new((Atom(25), Atom(99)))))))
            )))),
            Ok(Atom(25))
        );
        assert_eq!(
            net(Cell(Box::new((
                Atom(3),
                Cell(Box::new((Atom(531), Cell(Box::new((Atom(25), Atom(99)))))))
            )))),
            Ok(Cell(Box::new((Atom(25), Atom(99)))))
        );
    }

    #[test]
    fn test_hax() {
        assert_eq!(
            hax(Cell(Box::new((
                Atom(2),
                Cell(Box::new((Atom(11), Cell(Box::new((Atom(22), Atom(33)))))))
            )))),
            Ok(Cell(Box::new((Atom(11), Atom(33)))))
        );
        assert_eq!(
            hax(Cell(Box::new((
                Atom(3),
                Cell(Box::new((Atom(11), Cell(Box::new((Atom(22), Atom(33)))))))
            )))),
            Ok(Cell(Box::new((Atom(22), Atom(11)))))
        );
        assert_eq!(
            hax(Cell(Box::new((
                Atom(5),
                Cell(Box::new((
                    Atom(11),
                    Cell(Box::new((Cell(Box::new((Atom(22), Atom(33)))), Atom(44))))
                )))
            )))),
            Ok(Cell(Box::new((
                Cell(Box::new((Atom(22), Atom(11)))),
                Atom(44)
            ))))
        )
    }
}
