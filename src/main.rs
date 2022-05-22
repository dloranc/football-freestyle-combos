fn main() {
    let tricks = vec![
        "atw in", "atw out", "htw in", "half atw",
        // "crossover",
        // "x-over",
        // "reverse crossover",
        // "half atw in",
        // "htw out",
        // "htatw", "hmatw", "tatw", "matw", "atatw", "amatw", "latw", "alatw", "aatw",
    ];

    let tricks_in_combo: u32 = 10;

    for n in 1..=tricks_in_combo {
        let combos = generate_n_trick_combinations(n, &tricks);

        println!("{}-trick combos:", n);

        for combo in combos {
            println!("{}", combo);
        }

        println!();
    }
}

fn generate_n_trick_combinations(n: u32, tricks: &[&str]) -> Vec<String> {
    let mut combos = Vec::new();

    if n == 1 {
        for trick in tricks {
            combos.push(trick.to_string());
        }
    } else {
        for trick in tricks {
            let mut combo = Vec::new();
            combo.push(trick.to_string());

            let mut sub_combos = generate_n_trick_combinations(n - 1, tricks);

            for sub_combo in &mut sub_combos {
                combo.push(sub_combo.clone());
                combos.push(combo.join(", "));
                combo.pop();
            }
        }
    }

    combos
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_combinations(tricks_len: usize, n: u32) -> usize {
        (tricks_len as u32).pow(n as u32) as usize
    }

    #[test]
    fn test_generate_n_trick_combinations() {
        let tricks = vec![
            "atw in", "atw out", "htw in", "half atw",
        ];

        let combos = generate_n_trick_combinations(1, &tricks);

        assert_eq!(combos.len(), count_combinations(tricks.len(), 1));

        for combo in combos {
            assert_eq!(combo.split(", ").count(), 1);
        }

        let combos = generate_n_trick_combinations(2, &tricks);

        assert_eq!(combos.len(), count_combinations(tricks.len(), 2));

        for combo in combos {
            assert_eq!(combo.split(", ").count(), 2);
        }

        let combos = generate_n_trick_combinations(3, &tricks);

        assert_eq!(combos.len(), count_combinations(tricks.len(), 3));

        for combo in combos {
            assert_eq!(combo.split(", ").count(), 3);
        }

        let combos = generate_n_trick_combinations(4, &tricks);

        assert_eq!(combos.len(), count_combinations(tricks.len(), 4));

        for combo in combos {
            assert_eq!(combo.split(", ").count(), 4);
        }

        let combos = generate_n_trick_combinations(5, &tricks);

        assert_eq!(combos.len(), count_combinations(tricks.len(), 5));
    }
}
