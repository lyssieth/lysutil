pub const I: usize = 1;
pub const V: usize = 5;
pub const X: usize = 10;
pub const L: usize = 50;
pub const C: usize = 100;
pub const D: usize = 500;
pub const M: usize = 1000;

pub fn from_numerals(num: &str) -> usize {
    let chars: Vec<&str> = num.split("").collect();
    let mut numbers: Vec<&usize> = vec![];
    let mut _sum = 0usize;

    for x in chars {
        let n = match x {
            "i" | "I" => &I,
            "v" | "V" => &V,
            "x" | "X" => &X,
            "l" | "L" => &L,
            "c" | "C" => &C,
            "d" | "D" => &D,
            "m" | "M" => &M,
            _ => &0,
        };

        numbers.push(n);
    }

    // for i in 0..numbers.len() {
    //     break;
    // }

    0
}

pub fn to_numerals(_num: &usize) -> String {
    "".to_string()
}
