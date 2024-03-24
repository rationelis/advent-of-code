use utils::read_input_file;

use std::iter::Peekable;
use std::str::Chars;
use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Int(i32),
}

#[derive(Debug, PartialEq)]
enum Token {
    OpenBracket,
    CloseBracket,
    Comma,
    Number(i32),
}

struct Tokens<'a> {
    stream: Peekable<Chars<'a>>,
}

impl<'a> Tokens<'a> {
    fn new(input: &'a str) -> Self {
        Tokens {
            stream: input.chars().peekable(),
        }
    }

    fn next(&mut self) -> Option<Token> {
        match self.stream.peek() {
            Some(&c) => {
                match c {
                    '[' => {
                        self.stream.next();
                        Some(Token::OpenBracket)
                    },
                    ']' => {
                        self.stream.next();
                        Some(Token::CloseBracket)
                    },
                    ',' => {
                        self.stream.next();
                        Some(Token::Comma)
                    },
                    '0'..='9' => {
                        let mut number = 0;
                        while let Some(&c) = self.stream.peek() {
                            if c.is_digit(10) {
                                number = number * 10 + c.to_digit(10).unwrap() as i32;
                                self.stream.next();
                            } else {
                                break;
                            }
                        }
                        Some(Token::Number(number))
                    },
                    _ => None,
                }
            },
            None => None,
        }
    }
}

#[test]
fn test_tokens() {
    let mut tokens = Tokens::new("[1,2,3]");
    assert_eq!(tokens.next(), Some(Token::OpenBracket));
    assert_eq!(tokens.next(), Some(Token::Number(1)));
    assert_eq!(tokens.next(), Some(Token::Comma));
    assert_eq!(tokens.next(), Some(Token::Number(2)));
    assert_eq!(tokens.next(), Some(Token::Comma));
    assert_eq!(tokens.next(), Some(Token::Number(3)));
    assert_eq!(tokens.next(), Some(Token::CloseBracket));
    assert_eq!(tokens.next(), None);
}

fn parse_list_contents(tokens: &mut Tokens) -> Vec<Packet> {
    let mut list = Vec::new();
    loop {
        match tokens.next() {
            Some(Token::CloseBracket) => break,
            Some(Token::Number(n)) => list.push(Packet::Int(n)),
            Some(Token::OpenBracket) => list.push(Packet::List(parse_list_contents(tokens))),
            Some(Token::Comma) => continue,
            None => break,
        }
    }
    list
}

fn parse_packet(tokens: &mut Tokens) -> Packet {
    match tokens.next() {
        Some(Token::OpenBracket) => {
            Packet::List(parse_list_contents(tokens))
        },
        Some(Token::CloseBracket) => {
            Packet::List(Vec::new())
        },
        Some(Token::Number(n)) => {
            Packet::Int(n)
        },
        _ => panic!("Invalid token"),
    }
}

#[test]
fn test_parse_packet() {
    let mut tokens = Tokens::new("[1,2,3]");
    assert_eq!(parse_packet(&mut tokens), Packet::List(vec![Packet::Int(1), Packet::Int(2), Packet::Int(3)]));
}

fn compare_packets(p1: &Packet, p2: &Packet) -> Ordering {
    match (p1, p2) {
        (&Packet::Int(lint), &Packet::Int(rint)) => lint.cmp(&rint),
        (&Packet::List(ref llist), &Packet::List(ref rlist)) => {
            let mut lelems = llist.iter();
            let mut relems = rlist.iter();

            loop {
                let left = lelems.next();
                let right = relems.next();

                match (left, right) {
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (None, None) => return Ordering::Equal,
                    (Some(left), Some(right)) => match compare_packets(left, right) {
                        Ordering::Equal => continue,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                    },
                }
            }
        }
        (&Packet::Int(lint), right_packet_list) => {
            compare_packets(&Packet::List(vec![Packet::Int(lint)]), right_packet_list)
        }
        (left_packet_list, &Packet::Int(rint)) => {
            compare_packets(left_packet_list, &Packet::List(vec![Packet::Int(rint)]))
        }
    }
}

fn parse_input(lines: &[String]) -> Vec<Packet> {
    lines.iter().filter(|line| !line.is_empty()).map(|line| {
        let mut tokens = Tokens::new(line);
        parse_packet(&mut tokens)
    }).collect()
}

fn main() {
    let lines = read_input_file("input.txt").unwrap();
    
    let mut packets = parse_input(&lines);

    let indices_left_smaller = Itertools::tuples(packets.iter())
        .enumerate()
        .filter(|(_, (p1, p2))| compare_packets(p1, p2) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    println!("Part 1: {}", indices_left_smaller);

    let divider_1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    packets.extend(vec![divider_1.clone(), divider_2.clone()]);

    let sorted_packets = packets.iter().sorted_by(|p1, p2| compare_packets(p1, p2));

    let index_2 = sorted_packets.clone().position(|packet| compare_packets(packet, &divider_1) == Ordering::Equal).unwrap() + 1;
    let index_6 = sorted_packets.clone().position(|packet| compare_packets(packet, &divider_2) == Ordering::Equal).unwrap() + 1;

    println!("Part 2: {}", index_2 * index_6);
}
