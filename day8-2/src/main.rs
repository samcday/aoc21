use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    let digits = vec![
        "abcefg",
        "cf",
        "acdeg",
        "acdfg",
        "bcdf",
        "abdfg",
        "abdefg",
        "acf",
        "abcdefg",
        "abcdfg",
    ];

    let mut input: Vec<(Vec<String>, Vec<String>)> = stdin.lock().lines()
        .map(|line| {
            let line = line.unwrap();
            println!("{}", line);
            let mut it = line.split(" | ");
            (
                it.next().unwrap().split(" ").map(|x| x.to_string()).collect(),
                it.next().unwrap().split(" ").map(|x| x.to_string()).collect()
            )
        })
        .collect();

    let mut sum = 0;

    for (signal, output) in input {
        let mut decoder: HashMap<char, Vec<char>> = HashMap::new();

        for digit in "0123456789".chars() {
            decoder.insert(digit, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
        }

        for str in &signal {
            let (is_uniq, digit) = match str.len() {
                2 => (true, '1'),
                3 => (true, '7'),
                4 => (true, '4'),
                7 => (true, '8'),
                // 2|3|4|7 => {
                //     println!("uniq: {}", str);
                //     uniq_digit_count += 1;
                // },
                _ => (false, '0')
            };
            if is_uniq {
                // println!("hmm: {} {}", digit, str);
                decoder.get_mut(&digit).unwrap().retain(|x| str.contains(&x.to_string()))
            }
        }

        let digit_1 = decoder.get(&'1').unwrap().clone();
        let digit_4 = decoder.get(&'4').unwrap().clone();
        let digit_7 = decoder.get(&'7').unwrap().clone();
        let digit_8 = decoder.get(&'8').unwrap().clone();

        // Digits 0, 6 and 9 all have 6 segments. Only digit 6 is missing a segment that digit 1 has.
        let digits_069 = signal.iter()
            .filter(|v| v.len() == 6)
            .collect::<Vec<&String>>();

        let digit_6 = digits_069.iter()
            .filter(|v| {
                !digit_1.iter().all(|x| v.contains(&x.to_string()))
            })
            .collect::<Vec<&&String>>();
        assert_eq!(digit_6.len(), 1);
        let digit_6 = digit_6.first().unwrap().chars().collect::<Vec<char>>();

        // Same as before, but digit 0 misses a segment that digit 4 has.
        let digit_0 = digits_069.iter()
            .filter(|v| {
                !digit_6.iter().all(|x| v.contains(&x.to_string())) &&
                    !digit_4.iter().all(|x| v.contains(&x.to_string()))
            })
            .collect::<Vec<&&String>>();
        assert_eq!(digit_0.len(), 1);
        let digit_0 = digit_0.first().unwrap().chars().collect::<Vec<char>>();

        // Continuing this awful algorithmic/code spaghetti, digit 9 is whatever's left.
        let digit_9 = digits_069.iter().filter(|v| {
            ***v != digit_0.iter().collect::<String>() &&
                ***v != digit_6.iter().collect::<String>()
        }).collect::<Vec<&&String>>();
        assert_eq!(digit_9.len(), 1);
        let digit_9 = digit_9.first().unwrap().chars().collect::<Vec<char>>();

        assert_eq!(digit_1.len(), 2);
        assert_eq!(digit_7.len(), 3);
        assert_eq!(decoder.get(&'4').unwrap().len(), 4);
        assert_eq!(decoder.get(&'8').unwrap().len(), 7);

        let mut signals: HashMap<char, char> = HashMap::new();

        let scrambled_a =
        // Whichever extra signal digit 7 has that the 1 doesn't, is 'a'.
        {
            let mut digit_7 = digit_7.clone();
            digit_7.retain(|x| !digit_1.contains(x));
            assert_eq!(digit_7.len(), 1);
            signals.insert(digit_7[0], 'a');
            digit_7[0]
        };

        // Whicever extra signal digit 8 has that 6 doesn't, is 'c'.
        let scrambled_c = {
            let mut digit_8 = digit_8.clone();
            digit_8.retain(|x| !digit_6.contains(x));
            assert_eq!(digit_8.len(), 1);
            signals.insert(digit_8[0], 'c');
            digit_8[0]
        };


        // Remove signal c from digit 1 to determine signal f.
        {
            let mut digit_1 = digit_1.clone();
            digit_1.retain(|x| *x != scrambled_c);
            assert_eq!(digit_1.len(), 1);
            signals.insert(digit_1[0], 'f');
        }

        // Descent into madness.
        let mut deduction = digit_8.clone();
        deduction.retain(|v| !digit_9.contains(v));
        assert_eq!(deduction.len(), 1);
        let scrambled_e = deduction[0];
        signals.insert(scrambled_e, 'e');

        let mut deduction = digit_8.clone();
        deduction.retain(|v| !digit_0.contains(v));
        assert_eq!(deduction.len(), 1);
        let scrambled_d = deduction[0];
        signals.insert(scrambled_d, 'd');

        let mut deduction = digit_0.clone();
        deduction.retain(|v| !digit_4.contains(v) && *v != scrambled_a && *v != scrambled_e);
        assert_eq!(deduction.len(), 1);
        signals.insert(deduction[0], 'g');

        let mut deduction = digit_4.clone();
        deduction.retain(|v| !digit_1.contains(v) && *v != scrambled_d);
        assert_eq!(deduction.len(), 1);
        signals.insert(deduction[0], 'b');

        // ... And so as you can clearly see....

        let empty: Vec<char> = vec![];

        println!("Signals: \n{}\ndigits:\n{}", signals.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join("\n"), vec![
            digit_0,
            digit_1,
            empty.clone(),
            empty.clone(),
            digit_4,
            empty.clone(),
            digit_6,
            digit_7,
            digit_8,
            digit_9,
        ].iter().enumerate().map(|(idx, v)| format!("{}: {}", idx, v.iter().collect::<String>())).collect::<Vec<String>>().join(", "));

        // for idx in 0..=9 {
        //     digits[idx].chars().map(|x| signals[])
        // }

        let result = output.iter()
            .map(|v| {
                // Decode.
                let mut decoded = v.chars().map(|v| signals[&v]).collect::<Vec<char>>();
                decoded.sort();
                println!("original: {} decoded: {:?}", v, decoded);
                let decoded = decoded.iter().collect::<String>();
                for (idx, str) in digits.iter().enumerate() {
                    if **str == decoded {
                        return idx.to_string();
                    }
                }
                panic!("onoes!");
            }).collect::<String>();

        let num = i32::from_str_radix(&result, 10).unwrap();
        println!("'{}' {}", result, num);
        sum += num;
    }
    println!("SUM: {}", sum);
}
