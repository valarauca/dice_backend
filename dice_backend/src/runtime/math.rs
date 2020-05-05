use num_rational::Ratio;
pub type Rational = Ratio<usize>;

pub fn dice_roll(expected_output: usize, num_dice: usize, sides_on_dice: usize) -> Ratio<i128> {
    let expected_output: i128 = expected_output as i128;
    let num_dice: i128 = num_dice as i128;
    let sides_on_dice: i128 = sides_on_dice as i128;

    let mut k_max = expected_output
        .checked_sub(num_dice)
        .unwrap_or_else(|| 0i128)
        / sides_on_dice;
    if k_max == 0 {
        k_max = 1;
    }

    let lambda = |k: i128| -> i128 {
        let sign = if (k & 0x01i128) == 0 { 1i128 } else { -1i128 };
        let comb1 = combinations(num_dice, k);
        let comb2_top = expected_output
            .checked_sub(sides_on_dice * k)
            .unwrap_or_else(|| 0i128)
            .checked_sub(1)
            .unwrap_or_else(|| 0i128);
        let comb2_bot = num_dice.checked_sub(1).unwrap_or_else(|| 0i128);
        let comb2 = combinations(comb2_top, comb2_bot);
        sign * comb1 * comb2
    };
    let output = (0i128..=k_max).map(lambda).sum::<i128>();
    Ratio::new(output, sides_on_dice.pow(num_dice as u32) as i128)
}

#[test]
fn test_dice_roll() {
    struct TestSet {
        expected: usize,
        num: usize,
        sides: usize,
        output: Ratio<i128>,
    }
    impl TestSet {
        fn exec_test(&self) {
            let given = dice_roll(self.expected, self.num, self.sides);
            if given != self.output {
                panic!(
                    "expected prob:'{:?}' found prob:'{:?}' for a sum of {} with N={} d{}",
                    self.output, given, self.expected, self.num, self.sides
                );
            }
        }
    }

    let tests: &[TestSet] = &[
        /*
         * All results for 1 d6
         *
         */
        TestSet {
            expected: 1,
            num: 1,
            sides: 6,
            output: Ratio::new(1, 6),
        },
        TestSet {
            expected: 2,
            num: 1,
            sides: 6,
            output: Ratio::new(1, 6),
        },
        TestSet {
            expected: 3,
            num: 1,
            sides: 6,
            output: Ratio::new(1, 6),
        },
        TestSet {
            expected: 4,
            num: 1,
            sides: 6,
            output: Ratio::new(1, 6),
        },
        TestSet {
            expected: 5,
            num: 1,
            sides: 6,
            output: Ratio::new(1, 6),
        },
        TestSet {
            expected: 6,
            num: 1,
            sides: 6,
            output: Ratio::new(1, 6),
        },
    ];

    for test in tests {
        test.exec_test()
    }

    drop(tests);
}

#[inline(always)]
fn combinations(n: i128, k: i128) -> i128 {
    if k > n {
        0
    } else if k == n {
        1
    } else {
        factorial(n) / (factorial(k) * factorial(n - k))
    }
}

#[inline(always)]
fn factorial(x: i128) -> i128 {
    if x <= 1 {
        1i128
    } else {
        (1..=x).fold(1i128, |a, b| a * b)
    }
}
