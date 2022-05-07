use clap::Arg;
use clap::Command;
use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Default)]
struct Rechenraetsel {
    cache: HashMap<Box<[u8]>, (BTreeSet<i64>, BTreeSet<i64>)>,
    cache_mul: HashMap<Box<[u8]>, (BTreeSet<i64>, BTreeSet<i64>)>,
}

impl Rechenraetsel {
    fn possible_results(&mut self, digits: &[u8]) -> (BTreeSet<i64>, BTreeSet<i64>) {
        if digits.len() == 0 {
            return ([0].into_iter().collect(), BTreeSet::new());
        }
        match self.cache.get(digits) {
            Some(r) => return r.clone(),
            None => {},
        }
        let result = self.possible_results_uncached(digits);
        self.cache.insert(digits.to_owned().into_boxed_slice(), result.clone());
        result
    }
    // Split off the last summand.
    fn possible_results_uncached(&mut self, digits: &[u8]) -> (BTreeSet<i64>, BTreeSet<i64>) {
        let mut new_possible = BTreeSet::new();
        let mut new_duplicates = BTreeSet::new();
        for sep in 0..digits.len() {
            let (possible1, duplicates1) = self.possible_results(&digits[..sep]);
            let (possible2, duplicates2) = self.possible_results_mul(&digits[sep..]);
            new_duplicates.extend(possible1.iter().copied().flat_map(|v1| duplicates2.iter().copied().map(move |v2| v1 + v2)));
            new_duplicates.extend(duplicates1.iter().copied().flat_map(|v1| possible2.iter().copied().map(move |v2| v1 + v2)));
            new_duplicates.extend(duplicates1.iter().copied().flat_map(|v1| duplicates2.iter().copied().map(move |v2| v1 + v2)));

            if sep != 0 {
                new_duplicates.extend(possible1.iter().copied().flat_map(|v1| duplicates2.iter().copied().map(move |v2| v1 - v2)));
                new_duplicates.extend(duplicates1.iter().copied().flat_map(|v1| possible2.iter().copied().map(move |v2| v1 - v2)));
                new_duplicates.extend(duplicates1.iter().copied().flat_map(|v1| duplicates2.iter().copied().map(move |v2| v1 - v2)));
            }

            let possible_plus: BTreeSet<i64> = possible1.iter().copied().flat_map(|v1| possible2.iter().copied().map(move |v2| v1 + v2)).collect();
            let possible_minus: BTreeSet<i64> = if sep != 0 {
                possible1.iter().copied().flat_map(|v1| possible2.iter().copied().map(move |v2| v1 - v2)).collect()
            } else {
                BTreeSet::new()
            };

            new_duplicates.extend(new_possible.intersection(&possible_plus));
            if sep != 0 {
                new_duplicates.extend(possible_plus.intersection(&possible_minus));
                new_duplicates.extend(new_possible.intersection(&possible_minus));
            }

            let new_possible_prev = new_possible;
            new_possible = BTreeSet::new();
            new_possible.extend(possible_plus.iter().copied().filter(|p| !new_duplicates.contains(p)));
            new_possible.extend(new_possible_prev.iter().copied().filter(|p| !new_duplicates.contains(p)));
            if sep != 0 {
                new_possible.extend(possible_minus.iter().copied().filter(|p| !new_duplicates.contains(p)));
            }
        }
        (new_possible, new_duplicates)
    }

    fn possible_results_mul(&mut self, digits: &[u8]) -> (BTreeSet<i64>, BTreeSet<i64>) {
        if digits.len() == 0 {
            return ([1].into_iter().collect(), BTreeSet::new());
        }
        match self.cache_mul.get(digits) {
            Some(r) => return r.clone(),
            None => {},
        }
        let result = self.possible_results_mul_uncached(digits);
        self.cache_mul.insert(digits.to_owned().into_boxed_slice(), result.clone());
        result
    }
    fn possible_results_mul_uncached(&mut self, digits: &[u8]) -> (BTreeSet<i64>, BTreeSet<i64>) {
        let (possible, duplicates) = self.possible_results_mul(&digits[..digits.len() - 1]);
        let next_digit = digits[digits.len() - 1] as i64;
        if next_digit == 0 {
            if duplicates.len() != 0 || possible.len() != 1 {
                return (BTreeSet::new(), [0].into_iter().collect());
            } else {
                return ([0].into_iter().collect(), BTreeSet::new());
            }
        }
        let new_possible_mul: BTreeSet<i64> = possible.iter().copied().map(|p| p * next_digit).collect();
        let new_possible_div: BTreeSet<i64> = possible.iter().copied().filter_map(|p| if p % next_digit == 0 { Some(p / next_digit) } else { None }).collect();
        let mut new_duplicates: BTreeSet<i64> = BTreeSet::new();
        new_duplicates.extend(duplicates.iter().copied().map(|p| p * next_digit));
        new_duplicates.extend(duplicates.iter().copied().filter_map(|p| if p % next_digit == 0 { Some(p / next_digit) } else { None }));
        new_duplicates.extend(new_possible_mul.intersection(&new_possible_div).copied());
        (new_possible_mul.symmetric_difference(&new_possible_div).copied().filter(|p| !new_duplicates.contains(p)).collect(), new_duplicates)
    }
}

fn main() {
    let matches = Command::new("rechenraetsel")
        .author("Tobias")
        .arg(Arg::new("digits")
            .value_name("DIGITS")
            .required(true)
            .help("Digits, one character per digit. E.g. \"443\".")
        )
        .arg(Arg::new("result")
            .value_name("RESULT")
            .help("Result to test for.")
        )
        .get_matches();

    let mut digits = Vec::new();
    for c in matches.value_of("digits").unwrap().chars() {
        if !('0' <= c && c <= '9') {
            panic!("invalid digit {}", c);
        }
        digits.push(c as u8 - '0' as u8);
    }
    let result: Option<i64> = if matches.is_present("result") {
        Some(matches.value_of_t_or_exit("result"))
    } else {
        None
    };

    let mut rechenraetsel = Rechenraetsel::default();
    let (possible, duplicates) = rechenraetsel.possible_results(&digits);

    if let Some(r) = result {
        if possible.contains(&r) {
            println!("unique");
        } else if duplicates.contains(&r) {
            println!("duplicate");
        } else {
            println!("impossible");
        }
    } else {
        println!("uniques: {:?}", possible);
        println!("duplicates: {:?}", duplicates);
    }
}
