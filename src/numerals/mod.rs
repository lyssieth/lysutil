use std::cmp::Ordering;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Numerals {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl Numerals {
    fn value(&self) -> usize {
        match self {
            Numerals::I => 1,
            Numerals::V => 5,
            Numerals::X => 10,
            Numerals::L => 50,
            Numerals::C => 100,
            Numerals::D => 500,
            Numerals::M => 1000,
        }
    }
}

impl PartialOrd for Numerals {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Numerals {
    fn cmp(&self, other: &Self) -> Ordering {
        let val = self.value();
        let other_val = other.value();

        val.cmp(&other_val)
    }
}

pub fn from_numerals(num: &str) -> usize {
    let chars: Vec<&str> = num.split("").collect();
    let mut numbers: Vec<Numerals> = vec![];
    let mut sum = 0usize;

    for x in chars {
        let n = match x {
            "i" | "I" => Some(Numerals::I),
            "v" | "V" => Some(Numerals::V),
            "x" | "X" => Some(Numerals::X),
            "l" | "L" => Some(Numerals::L),
            "c" | "C" => Some(Numerals::C),
            "d" | "D" => Some(Numerals::D),
            "m" | "M" => Some(Numerals::M),
            _ => None,
        };

        if let Some(n) = n {
            numbers.push(n);
        }
    }

    let mut idx = 0;
    let mut tmp_sum = 0usize;

    let peek = |idx| -> Option<Numerals> {
        let i = idx + 1;
        if i >= numbers.len() {
            None
        } else {
            Some(numbers[i])
        }
    };
    let past = |idx| -> Option<Numerals> {
        let i = if 1 > idx { 0 } else { idx - 1 };
        if i >= numbers.len() {
            None
        } else {
            Some(numbers[i])
        }
    };
    let take = |idx| -> Numerals { numbers[idx] };

    loop {
        let current = take(idx);
        let next = peek(idx);

        println!("Current is: {:?}", current);
        println!("Next is: {:?}", next);

        dbg!(&idx, &tmp_sum, &sum);

        match next {
            Some(next) => {
                println!("some next");
                match current.cmp(&next) {
                    Ordering::Less => {
                        println!("current < next");
                        tmp_sum += current.value();
                        idx += 1;
                    }
                    Ordering::Equal => {
                        println!("current = next");
                        let prev = past(idx);
                        match prev {
                            Some(prev) => {
                                println!("some prev");
                                if prev < current {
                                    println!("prev < current");
                                    sum += current.value() - tmp_sum;
                                    tmp_sum = 0;
                                } else {
                                    println!("prev >= current");
                                    tmp_sum += current.value();
                                    sum += tmp_sum;
                                    tmp_sum = 0;
                                }
                            }
                            None => {
                                println!("no prev");
                                tmp_sum += current.value();
                                sum += tmp_sum;
                                tmp_sum = 0;
                            }
                        }
                        idx += 1;
                    }
                    Ordering::Greater => {
                        println!("current > next");
                        let prev = past(idx);
                        match prev {
                            Some(prev) => {
                                println!("some prev");
                                if prev < current {
                                    println!("prev < current");
                                    sum += current.value() - tmp_sum;
                                    tmp_sum = 0;
                                } else {
                                    println!("prev >= current");
                                    tmp_sum += current.value();
                                    sum += tmp_sum;
                                    tmp_sum = 0;
                                }
                            }
                            None => {
                                println!("no prev");
                                tmp_sum += current.value();
                                sum += tmp_sum;
                                tmp_sum = 0;
                            }
                        }
                        idx += 1;
                    }
                };
            }
            None => {
                println!("no next");
                let prev = past(idx);
                match prev {
                    Some(prev) => {
                        println!("some prev");
                        if prev < current {
                            println!("prev < current");
                            sum += current.value() - tmp_sum;
                        } else {
                            println!("prev >= current");
                            tmp_sum += current.value();
                            sum += tmp_sum;
                        }
                    }
                    None => {
                        println!("no prev");
                        tmp_sum += current.value();
                        sum += tmp_sum;
                    }
                }
                break;
            }
        }
    }

    sum
}

pub fn to_numerals(_num: &usize) -> String {
    "".to_string() //TODO: Write the opposite of from_numerals, then I can bump the version!!!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(1, from_numerals("I"));
    }
    #[test]
    fn two() {
        assert_eq!(2, from_numerals("II"));
    }
    #[test]
    fn three() {
        assert_eq!(3, from_numerals("III"));
    }
    #[test]
    fn four() {
        assert_eq!(4, from_numerals("IV"));
    }
    #[test]
    fn five() {
        assert_eq!(5, from_numerals("V"));
    }
    #[test]
    fn six() {
        assert_eq!(6, from_numerals("VI"));
    }
    #[test]
    fn seven() {
        assert_eq!(7, from_numerals("VII"));
    }
    #[test]
    fn eight() {
        assert_eq!(8, from_numerals("VIII"));
    }
    #[test]
    fn nine() {
        assert_eq!(9, from_numerals("IX"));
    }
    #[test]
    fn ten() {
        assert_eq!(10, from_numerals("X"));
    }

    #[test]
    fn mccxliii() {
        println!("1243");
        assert_eq!(1243, from_numerals("MCCXLIII"));
    }

    #[test]
    fn dclxvi() {
        assert_eq!(646, from_numerals("DCXLVI"))
    }
}
