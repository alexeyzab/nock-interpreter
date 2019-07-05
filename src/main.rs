#![feature(box_syntax, box_patterns)]

use std::fmt;
use Noun::*;

fn main() {
    println!(
        "{}",
        Cell(Box::new((Atom(12), Cell(Box::new((Atom(487), Atom(325)))))))
    );
}

#[derive(Debug, Eq, PartialEq)]
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
pub fn wut(input: Noun) -> Possibly<Noun> {
    match input {
        Cell(_) => Ok(Atom(0)),
        Atom(_) => Ok(Atom(1)),
    }
}

// `=`
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
pub fn lus(input: Noun) -> Possibly<Noun> {
    match input {
        Atom(n) => Ok(Atom(n + 1)),
        other => Err(Error(other)),
    }
}

// `/`
pub fn net(input: Noun) -> Possibly<Noun> {
    match input {
        Cell(box (n1, n2)) => match n1 {
            Atom(i) => match i {
                1 => Ok(n2),
                2 => match n2 {
                    Cell(box (inner_n1, _)) => Ok(inner_n1),
                    other => Err(Error(other)),
                },
                3 => match n2 {
                    Cell(box (_, inner_n2)) => Ok(inner_n2),
                    other => Err(Error(other)),
                },
                i if i > 3 => {
                    let rhs = net(Cell(Box::new((Atom(i / 2), n2))));
                    match rhs {
                        Ok(right_noun) => net(Cell(Box::new((Atom(2 + (i % 2)), right_noun)))),
                        Err(_) => return Err(Error(n1)),
                    }
                }
                _ => Err(Error(n1)),
            },
            other => Err(Error(other)),
        },
        Atom(_) => Err(Error(input)),
    }
}

// `#`
pub fn hax(_input: Noun) -> Possibly<Noun> {
    unimplemented!();
}

// `*`
pub fn tar(_input: Noun) -> Possibly<Noun> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_lus() {
        assert_eq!(lus(Atom(5)), Ok(Atom(6)));
        assert_eq!(
            lus(Cell(Box::new((Atom(0), Atom(1))))),
            Err(Error(Cell(Box::new((Atom(0), Atom(1))))))
        )
    }

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
}
