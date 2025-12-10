use std::fs;
use std::io::{BufReader, prelude::*};

// fn is_repeater(base: &[char], body: &[char]) -> bool {
//     let basestr: String = base.iter().collect();
//     let bodystr: String = body.iter().collect();
//     println!("Base: {}, Body: {}", basestr, bodystr);
//     println!("Lens: {}, {}", base.len(), body.len());
//     for n in (0..body.len()).step_by(base.len()) {
//         if *base != body[n..n + base.len()] {
//             return false;
//         }
//     }
//     return true;
// }
//
fn get_power(n: u64) -> u32 {
    for i in 0..15 {
        if 10u64.pow(i) <= n && n < 10u64.pow(i + 1) {
            return i;
        }
    }
    return 0;
}

fn even(n: u32) -> bool {
    n % 2 == 0
}

fn split_at_position(skuchars: Vec<char>, pos: usize) -> Result<(String, String), &'static str> {
    if pos >= skuchars.len() {
        return Err("oops");
    }
    let top = &skuchars[..pos];
    let bot = &skuchars[pos..];
    Ok((top.into_iter().collect(), bot.into_iter().collect()))
}

fn repeats_at_position(sku: u64, pos: usize) -> bool {
    // println!("sku {}, pos {}", sku, pos);
    let skuchars: Vec<char> = sku.to_string().chars().collect();
    let splitres = split_at_position(skuchars, pos);

    let (top, mut bot) = match splitres {
        Ok(x) => x,
        Err(_) => return false,
    };
    // println!("top {}, bot {}", top, bot);
    if top == bot {
        // println!("immediate match");
        return true;
    }
    if bot.len() < pos || top != bot[..pos] {
        // println!("immediate mismatch");
        return false;
    }
    let mut pieces: Vec<String> = vec![top];
    while bot.len() > pos {
        let splitres = split_at_position(bot.chars().collect(), pos);
        let split = match splitres {
            Ok(x) => x,
            Err(_) => return false,
        };
        let ntop = split.0;
        bot = split.1;
        pieces.push(ntop);
    }
    pieces.push(bot);
    // println!("{}", pieces.join("|"));
    let chunks: Vec<usize> = pieces.iter().map(|ns| ns.parse().unwrap()).collect();
    return chunks.iter().min() == chunks.iter().max();
}

fn is_metarepeater(sku: u64) -> bool {
    let pwr = get_power(sku);
    let halfpwr = if ((pwr + 1) / 2) >= 1 {
        (pwr + 1) / 2
    } else {
        1
    };
    // println!("{}, {}", pwr, halfpwr);
    for i in 1..=halfpwr as usize {
        // println!("i{}", i);
        if repeats_at_position(sku, i) {
            return true;
        }
    }
    false
}

pub fn find_valid() -> () {
    let rangegoo = fs::read_to_string("src/data/problem02").unwrap();
    let rangeblob = rangegoo.trim();
    let mut code_collect: u64 = 0;
    for range in rangeblob.split(",") {
        let bounds: Vec<u64> = range
            .split("-")
            .map(|b| b.parse::<u64>().unwrap())
            .collect();
        // println!("{}-{}", bounds[0], bounds[1]);
        for sku in bounds[0]..=bounds[1] {
            // let skustr = sku.to_string();
            // let chrs: Vec<char> = skustr.chars().collect();
            // let midpoint = chrs.len() / 2;
            if is_metarepeater(sku) {
                println!("sku: {}", sku);
                code_collect += sku;
            }
            // if sku == 6600066 {
            //     panic!("check it out")
            // }
        }
    }
    println!("Code is {}", code_collect);
}
