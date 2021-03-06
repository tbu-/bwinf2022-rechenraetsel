use clap::Arg;
use clap::Command;
use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Default)]
struct Rechenraetsel<const ALLOW_NEGATIVE: bool> {
    cache: HashMap<Box<[u8]>, (BTreeSet<i64>, BTreeSet<i64>)>,
    cache_mul: HashMap<Box<[u8]>, (BTreeSet<i64>, BTreeSet<i64>)>,
}

impl<const ALLOW_NEGATIVE: bool> Rechenraetsel<ALLOW_NEGATIVE> {
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
                if ALLOW_NEGATIVE {
                    new_duplicates.extend(possible1.iter().copied().flat_map(|v1| duplicates2.iter().copied().map(move |v2| v1 - v2)));
                    new_duplicates.extend(duplicates1.iter().copied().flat_map(|v1| possible2.iter().copied().map(move |v2| v1 - v2)));
                    new_duplicates.extend(duplicates1.iter().copied().flat_map(|v1| duplicates2.iter().copied().map(move |v2| v1 - v2)));
                } else {
                    new_duplicates.extend(possible1.iter().copied().flat_map(|v1| duplicates2.iter().copied().flat_map(move |v2| if v1 - v2 >= 0 { Some(v1 - v2) } else { None })));
                    new_duplicates.extend(duplicates1.iter().copied().flat_map(|v1| possible2.iter().copied().flat_map(move |v2| if v1 - v2 >= 0 { Some(v1 - v2) } else { None })));
                    new_duplicates.extend(duplicates1.iter().copied().flat_map(|v1| duplicates2.iter().copied().flat_map(move |v2| if v1 - v2 >= 0 { Some(v1 - v2) } else { None })));
                }
            }

            let possible_plus: BTreeSet<i64> = possible1.iter().copied().flat_map(|v1| possible2.iter().copied().map(move |v2| v1 + v2)).collect();
            let possible_minus: BTreeSet<i64> = if sep != 0 {
                if ALLOW_NEGATIVE {
                    possible1.iter().copied().flat_map(|v1| possible2.iter().copied().map(move |v2| v1 - v2)).collect()
                } else {
                    possible1.iter().copied().flat_map(|v1| possible2.iter().copied().filter_map(move |v2| if v1 - v2 >= 0 { Some(v1 - v2) } else { None })).collect()
                }
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
        let new_possible_div: BTreeSet<i64> = if digits.len() != 1 {
            possible.iter().copied().filter_map(|p| if p % next_digit == 0 { Some(p / next_digit) } else { None }).collect()
        } else {
            BTreeSet::new()
        };
        let mut new_duplicates: BTreeSet<i64> = BTreeSet::new();
        new_duplicates.extend(duplicates.iter().copied().map(|p| p * next_digit));
        if digits.len() != 1 {
            new_duplicates.extend(duplicates.iter().copied().filter_map(|p| if p % next_digit == 0 { Some(p / next_digit) } else { None }));
            new_duplicates.extend(new_possible_mul.intersection(&new_possible_div).copied());
        }
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
        .arg(Arg::new("no-negative-partials")
            .long("--no-negative-partials")
            .help("Disallow negative partial sums")
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

    let (possible, duplicates) = if matches.is_present("no-negative-partials") {
        Rechenraetsel::<false>::default().possible_results(&digits)
    } else {
        Rechenraetsel::<true>::default().possible_results(&digits)
    };

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
