#![feature(box_syntax, box_patterns)]

extern crate nom;
use nom::IResult;

use std::fmt;
use Noun::*;

fn main() {
    println!(
        "{}",
        cell(Atom(12), cell(cell(Atom(487), Atom(13)), Atom(325)))
    );
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Noun {
    Atom(u64),
    Cell(Box<(Noun, Noun)>),
}

pub fn cell(left_noun: Noun, right_noun: Noun) -> Noun {
    Cell(Box::new((left_noun, right_noun)))
}

impl fmt::Display for Noun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            Atom(a) => format!("{}", a),
            Cell(box (n1, n2)) => format!("[{} {}]", n1, n2),
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
                1 => Ok(cell(b, c)),
                2 => Ok(b),
                3 => Ok(c),
                i if i > 3 => {
                    let inner_net = net(cell(Atom(i / 2), cell(b, c)))?;
                    net(cell(Atom(2 + (i % 2)), inner_net))
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
                    let inner_net = cell(b, net(cell(Atom(n + 1), c.clone()))?);
                    hax(cell(Atom(n / 2), cell(inner_net, c)))
                }
                n if n % 2 == 1 => {
                    let inner_net = cell(net(cell(Atom(n - 1), c.clone()))?, b);
                    hax(cell(Atom((n - 1) / 2), cell(inner_net, c)))
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
            tar(cell(a.clone(), cell(b, c)))?,
            tar(cell(a, d))?,
        )))),

        Cell(box (a, Cell(box (Atom(0), b)))) => net(cell(b, a)),

        Cell(box (_a, Cell(box (Atom(1), b)))) => Ok(b),

        Cell(box (a, Cell(box (Atom(2), Cell(box (b, c)))))) => {
            tar(cell(tar(cell(a.clone(), b))?, tar(cell(a, c))?))
        }

        Cell(box (a, Cell(box (Atom(3), b)))) => wut(tar(cell(a, b))?),

        Cell(box (a, Cell(box (Atom(4), b)))) => lus(tar(cell(a, b))?),

        Cell(box (a, Cell(box (Atom(5), Cell(box (b, c)))))) => {
            tis(cell(tar(cell(a.clone(), b))?, tar(cell(a, c))?))
        }

        Cell(box (a, Cell(box (Atom(6), Cell(box (b, Cell(box (c, d)))))))) => tar(cell(
            a.clone(),
            tar(cell(
                cell(c, d),
                cell(
                    Atom(0),
                    tar(cell(
                        cell(Atom(2), Atom(3)),
                        cell(Atom(0), tar(cell(a, cell(Atom(4), cell(Atom(4), b))))?),
                    ))?,
                ),
            ))?,
        )),

        Cell(box (a, Cell(box (Atom(7), Cell(box (b, c)))))) => tar(cell(tar(cell(a, b))?, c)),

        Cell(box (a, Cell(box (Atom(8), Cell(box (b, c)))))) => {
            tar(cell(cell(tar(cell(a.clone(), b))?, a), c))
        }

        Cell(box (a, Cell(box (Atom(9), Cell(box (b, c)))))) => tar(cell(
            tar(cell(a, c))?,
            cell(Atom(2), cell(cell(Atom(0), Atom(1)), cell(Atom(0), b))),
        )),

        Cell(box (a, Cell(box (Atom(10), Cell(box (Cell(box (b, c)), d)))))) => {
            hax(cell(b, cell(tar(cell(a.clone(), c))?, tar(cell(a, d))?)))
        }

        Cell(box (a, Cell(box (Atom(11), Cell(box (Cell(box (_b, c)), d)))))) => tar(cell(
            cell(tar(cell(a.clone(), c))?, tar(cell(a, d))?),
            cell(Atom(0), Atom(3)),
        )),

        Cell(box (a, Cell(box (Atom(11), Cell(box (_b, c)))))) => tar(cell(a, c)),

        other => Err(Error(other)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wut() {
        assert_eq!(wut(cell(Atom(1), Atom(2))), Ok(Atom(0)));
        assert_eq!(wut(Atom(5)), Ok(Atom(1)));
    }

    #[test]
    fn test_tis() {
        assert_eq!(tis(cell(Atom(1), Atom(1))), Ok(Atom(0)));
        assert_eq!(tis(cell(Atom(1), Atom(2))), Ok(Atom(1)));
        assert_eq!(tis(Atom(5)), Err(Error(Atom(5))))
    }

    #[test]
    fn test_lus() {
        assert_eq!(lus(Atom(5)), Ok(Atom(6)));
        assert_eq!(
            lus(cell(Atom(0), Atom(1))),
            Err(Error(Cell(Box::new((Atom(0), Atom(1))))))
        )
    }

    #[test]
    fn test_net() {
        assert_eq!(
            net(cell(Atom(1), cell(Atom(531), cell(Atom(25), Atom(99))))),
            Ok(cell(Atom(531), cell(Atom(25), Atom(99))))
        );
        assert_eq!(
            net(cell(Atom(6), cell(Atom(531), cell(Atom(25), Atom(99))))),
            Ok(Atom(25))
        );
        assert_eq!(
            net(cell(Atom(3), cell(Atom(531), cell(Atom(25), Atom(99))))),
            Ok(cell(Atom(25), Atom(99)))
        );
    }

    #[test]
    fn test_hax() {
        assert_eq!(
            hax(cell(Atom(2), cell(Atom(11), cell(Atom(22), Atom(33))))),
            Ok(cell(Atom(11), Atom(33)))
        );
        assert_eq!(
            hax(cell(Atom(3), cell(Atom(11), cell(Atom(22), Atom(33))))),
            Ok(cell(Atom(22), Atom(11)))
        );
        assert_eq!(
            hax(cell(
                Atom(5),
                cell(Atom(11), cell(cell(Atom(22), Atom(33)), Atom(44)))
            )),
            Ok(cell(cell(Atom(22), Atom(11)), Atom(44)))
        )
    }
}
