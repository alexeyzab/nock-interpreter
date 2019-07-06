#![feature(box_syntax, box_patterns)]

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
        Cell(box (a, bc)) => match a {
            Atom(i) => match i {
                1 => Ok(bc),
                2 => match bc {
                    Cell(box (b, _)) => Ok(b),
                    Atom(_) => Err(Error(bc)),
                },
                3 => match bc {
                    Cell(box (_, c)) => Ok(c),
                    Atom(_) => Err(Error(bc)),
                },
                i if i > 3 => {
                    let inner_net = net(Cell(Box::new((Atom(i / 2), bc))))?;
                    net(Cell(Box::new((Atom(2 + (i % 2)), inner_net))))
                }
                _ => Err(Error(a)),
            },
            other => Err(Error(other)),
        },
        Atom(_) => Err(Error(input)),
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
        Cell(box (a, bc)) => match a {
            Cell(_) => Err(Error(a)),
            Atom(i) => match i {
                1 => match bc {
                    Atom(_) => Err(Error(bc)),
                    Cell(box (b, _)) => Ok(b),
                },
                n if n % 2 == 0 => match bc {
                    Atom(_) => Err(Error(bc)),
                    Cell(box (b, c)) => {
                        let c_copy = c.clone();
                        let inner_net = Cell(Box::new((b, net(Cell(Box::new((Atom(n + 1), c))))?)));
                        hax(Cell(Box::new((
                            Atom(n / 2),
                            Cell(Box::new((inner_net, c_copy))),
                        ))))
                    }
                },
                n if n % 2 == 1 => match bc {
                    Atom(_) => Err(Error(bc)),
                    Cell(box (b, c)) => {
                        let c_copy = c.clone();
                        let inner_net = Cell(Box::new((net(Cell(Box::new((Atom(n - 1), c))))?, b)));
                        hax(Cell(Box::new((
                            Atom((n - 1) / 2),
                            Cell(Box::new((inner_net, c_copy))),
                        ))))
                    }
                },
                _ => Err(Error(a)),
            },
        },
    }
}

// `*`
pub fn tar(_input: Noun) -> Possibly<Noun> {
    unimplemented!();
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
        assert_eq!(hax(Cell(Box::new((Atom(3), Cell(Box::new((Atom(11), Cell(Box::new((Atom(22), Atom(33))))))))))), Ok(Cell(Box::new((Atom(22), Atom(11))))));
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
