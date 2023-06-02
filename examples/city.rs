extern crate clige;
use clige::core::{
    buffer::{Buffer, PixelBuffer},
    color::{Color, Context},
    data::Pixel,
};
use std::{collections::HashMap, fmt::Debug};

/// # Reference
/// source: [L-Systems](https://liu.diva-portal.org/smash/get/diva2:1467574/FULLTEXT01.pdf)
/// S -> aSb
/// S -> ba
///
/// Starting iwth S:
///  Step 0: S == Apply rule 1
///  Step 1: aSb == Apply rule 1
///  Step 2: aaSbb we apply rule 1
///  Step 3: aaaSbbb we apply rule 2
///  Step 4: aaababbb
///
///  ___
///
/// ## Language:
///     A: Producer
///     B: Producer
///     F: Foreward
///     +: Turn Right
///     -: Turn Left
///     [: Marker
///     ]: Jump to last marker
///
///  ___
///
/// ## Grammer:
///     A -> `FB`
///     B -> `[+A][-A]`
///
///  ___
///
/// ## Sample:
///   Seed: `ABAAB`
///   Final Instruction Set: `F[+FB][-FB]`
///   Instructions:
///     - foreward
///     - mark location as m1
///     - turn right,
///     - foreward
///     - move to m1
///     - mark location as m2
///     - turn left
///     - foreward
///     - move to m2
///
///  ___
///
/// ## Sample Multistep
/// Step 0: `A`
/// Step 1: `FB`
/// Step 2: `F[+A][-A]`
/// Step 3: `F[+FB][-FB]`
/// Step 4: `F[+F[+A][-A]][-F[+A][-A]]`
/// Step 5: `F[+F[+FB][-FB]][-F[+FB][-FB]]`
/// Result: `F[+F[+F][-F]][-F[+F][-F]]`
fn main() {
    let mut city = PixelBuffer::default();
    // println!("{:?}", road!(dv));
    let grammer = grammer! {
        language: ['A', 'B', 'F', '[', ']', '+', '-'],
        'A' => "FB",
        'B' => "[+A]A[-A]",
    };

    let instructions = generator("A", grammer, 5);
    println!("{:?}", instructions);

    // x, y, direction => top, right, bottom, left => 0, 1, 2, 3
    let mut position: (i32, i32, i8) = (city.width() as i32 / 2, city.height() as i32 / 3, 2);
    let mut marks: Vec<(i32, i32, i8)> = Vec::new();
    for i in instructions.chars() {
        match i {
            'F' => {
                translate(&mut position, (city.width() as i32, city.height() as i32));
                city.set(position.0 as usize, position.1 as usize, '•'.into());
                println!("Move: {:?}", position)
            }
            '[' => {
                println!("Mark Location: {:?}", position);
                marks.push(position.clone())
            }
            ']' => {
                if marks.len() == 0 {
                    panic!("No marks found")
                }
                println!("Jump to Mark: {:?}", marks.last().unwrap());
                position = marks.pop().unwrap();
            }
            '-' => {
                position.2 -= 1;
                if position.2 < 0 {
                    position.2 = 3
                }
                println!("Turn Left: {:?}", position);
            }
            '+' => {
                position.2 += 1;
                if position.2 > 3 {
                    position.2 = 0
                }
                println!("Turn right: {:?}", position);
            }
            _ => panic!("Unkown instruction"),
        }
    }

    println!("{}", city.render().unwrap());
}

fn get_road(dir: i8) -> Pixel {
    match dir {
        0 => {
            road!(sv)
        }
        1 => {
            road!(sh)
        }
        2 => {
            road!(sv)
        }
        3 => {
            road!(sh)
        }
        _ => panic!("Unkown direction: {}", dir)
    }
}

fn translate(pos: &mut (i32, i32, i8), max: (i32, i32)) {
    match pos.2 {
        0 => {
            pos.1 -= 1;
            pos.1 = pos.1.max(0);
        }
        1 => {
            pos.0 += 1;
            pos.0 = pos.0.min(max.0 as i32 - 1);
        }
        2 => {
            pos.1 += 1;
            pos.1 = pos.1.min(max.1 as i32 - 1);
        }
        3 => {
            pos.0 -= 1;
            pos.0 = pos.0.max(0);
        }
        _ => panic!("Unkown direction: {}", pos.2)
    }
}

///
/// Generates a string by replacing producers with their values
///
/// # Args:
/// - seed: starting string
/// - grammer: set of rules to be applied
/// - iterations: number of replacement iterations
fn generator(seed: &str, grammer: Grammer, iterations: u8) -> String {
    let mut result = seed.to_string();

    for _ in 0..iterations {
        let mut buffer = String::new();
        for c in result.chars() {
            if grammer.contains(&c) {
                match grammer.get(c) {
                    Some(value) => buffer.push_str(value.as_str()),
                    None => buffer.push(c),
                }
            } else {
                buffer.push(c)
            }
        }
        result = buffer;
        println!("{:?}", result);
    }

    result
        .chars()
        .filter(|c| !grammer.contains(c))
        .collect::<String>()
}

/// Immutable HashMap of producer to value
#[derive(Debug)]
struct Grammer(HashMap<char, String>);
impl Grammer {
    /// Validate that the characters in the rules are in the langauge
    fn validate(rules: &HashMap<char, String>, language: &Vec<char>) {
        for p in rules.keys() {
            if !language.contains(p) {
                panic!(
                    "Mismatch Producers: Expected one of {:?} but was \"{}\"",
                    language, p
                );
            }
        }

        for (p, v) in rules.iter() {
            if !v.chars().all(|c| language.contains(&c)) {
                panic!(
                    "Mismatch values: '{}' => \"{}\", value contains chars not in in language {:?}",
                    p, v, language
                );
            }
        }
    }

    pub fn new(language: Vec<char>, rules: HashMap<char, String>) -> Self {
        Grammer::validate(&rules, &language);
        Grammer(rules)
    }

    pub fn get(&self, key: char) -> Option<&String> {
        self.0.get(&key)
    }

    pub fn contains(&self, key: &char) -> bool {
        self.0.contains_key(key)
    }
}

/// Construct a grammer
///
/// # Args
/// - language: List of characters in the language of the format `['A', ...]` or `language: ['A',
/// ...]`
/// - list of producers to values of the format: `producer => value`
///
/// # Example
/// ```
/// grammer! {
///     ['S', 'a', 'b'],
///     'S' => "aSb"
/// }
///
/// // or
///
/// grammer! {
///     language: ['S', 'B', 'a'],
///     'S' => "aB",
///     'B' => "Sa",
/// }
/// ```
#[macro_export]
macro_rules! grammer {
    {$(language: )*[$($lang: literal),*$(,)*], $($producer: literal => $value: literal),*$(,)*} => {
        Grammer::new(
            vec![$($lang,)*],
            HashMap::from([$(($producer, $value.to_string()),)*]),
        )
    };
}

/// Constructer a road.
///
/// Either vertical or horizontal; solid lines or dotted lines
///
/// # Options
/// - `sv`: Solid Vertical
/// - `sh`: Solid Horizontal
/// - `dv`: Dotted Vertical
/// - `dh`: Dotted Horizontal
#[macro_export]
macro_rules! road {
    (sv) => {
        Pixel {
            color: Color::default(),
            value: '║',
        }
    };
    (sh) => {
        Pixel {
            color: Color::default(),
            value: '═',
        }
    };
    (dv) => {
        Pixel {
            color: Color::default(),
            value: '┆',
        }
    };
    (dh) => {
        Pixel {
            color: Color::default(),
            value: '┄',
        }
    };
}
