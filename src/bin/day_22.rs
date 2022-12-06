use adventofcode_2019::get_input;

fn main() {
    let input = get_input().unwrap();

    let deck: Vec<usize> = (0..10007).collect();

    let deck = shuffle_cards(deck, input.trim());

    for (i, card_val) in deck.iter().enumerate() {
        if *card_val == 2019 {
            println!("Part 1: {}", i);
        }
    }

    // yeah, right
    // let mut deck: Vec<usize> = (0..119315717514047).collect();
    // for _ in 0usize..101741582076661 {
    //     deck = shuffle_cards(deck, input.trim())
    // }
}

fn shuffle_cards(mut deck: Vec<usize>, shuffle_steps: &str) -> Vec<usize> {
    let deck_size = deck.len();
    let deck_isize = deck_size as isize;
    let mut start: isize = 0;
    let mut step: isize = 1;
    let mut stack = vec![0; deck_size];

    for line in shuffle_steps.lines() {
        if cfg!(feature = "debug") {
            println!("{line}");
        }

        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("deal") && line.ends_with("stack") {
            step *= -1;
            start += step;
            if start >= deck_isize {
                start -= deck_isize;
            } else if start < 0 {
                start += deck_isize;
            }
        } else if line.starts_with("deal") {
            let increment: isize = line.split_whitespace().last().unwrap().parse().unwrap();

            let mut i = 0;
            let mut j = start;
            for _ in 0..deck_size {
                stack[i as usize] = deck[j as usize];

                i = (i + increment) % deck_isize;
                j += step;

                if j >= deck_isize {
                    j -= deck_isize;
                } else if j < 0 {
                    j += deck_isize;
                }
            }

            let tmp = stack;
            stack = deck;
            deck = tmp;
            start = 0;
            step = 1;
        } else if line.starts_with("cut") {
            let size: isize = line.split_whitespace().last().unwrap().parse().unwrap();
            let mut tmp = start + (size * step);

            if tmp >= deck_isize {
                tmp -= deck_isize;
            } else if tmp < 0 {
                tmp += deck_isize;
            }

            start = tmp;
        } else {
            panic!("unrecogized technique {}", line)
        }

        if cfg!(feature = "debug") {
            dbg!(start, step);
            let mut i = start;
            for _ in 0..deck_size {
                print!("{} ", deck[i as usize]);
                i += step;
                if i >= deck_isize {
                    i -= deck_isize;
                } else if i < 0 {
                    i += deck_isize;
                }
            }
            println!();
        }
    }

    let mut i = start;
    for j in 0..deck_size {
        stack[j] = deck[i as usize];
        i += step;
        if i >= deck_isize {
            i -= deck_isize;
        } else if i < 0 {
            i += deck_isize;
        }
    }

    stack
}

#[test]
fn dealing_into_stack_reversible() {
    assert_eq!(
        shuffle_cards(
            (0..10).rev().collect(),
            r"
deal into new stack
            "
        ),
        (0..10).collect::<Vec<_>>()
    );

    assert_eq!(
        shuffle_cards(
            (0..10).collect(),
            r"
deal into new stack
deal into new stack
            "
        ),
        (0..10).collect::<Vec<_>>()
    );

    assert_eq!(
        shuffle_cards(
            (0..10).collect(),
            r"
cut 7
deal into new stack
deal into new stack
            "
        ),
        shuffle_cards(
            (0..10).collect(),
            r"
cut 7
            "
        ),
    );
}

#[test]
fn cut_works() {
    assert_eq!(
        shuffle_cards((0..10).collect(), "cut 3"),
        vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]
    );

    assert_eq!(
        shuffle_cards((0..13).collect(), "cut 3"),
        vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0, 1, 2]
    );

    assert_eq!(
        shuffle_cards(
            (0..10).collect(),
            r"
deal into new stack
cut 3
            "
        ),
        vec![6, 5, 4, 3, 2, 1, 0, 9, 8, 7,]
    );

    assert_eq!(
        shuffle_cards(
            (0..10).collect(),
            r"
deal into new stack
cut -4
            "
        ),
        vec![3, 2, 1, 0, 9, 8, 7, 6, 5, 4,]
    );
}

#[test]
fn deal_with_increment_works() {
    assert_eq!(
        shuffle_cards(
            (0..10).collect(),
            r"
deal into new stack
deal with increment 7
            "
        ),
        vec![9, 6, 3, 0, 7, 4, 1, 8, 5, 2]
    );
}

#[test]
fn examples_work() {
    assert_eq!(
        shuffle_cards(
            (0..10).collect(),
            r"
deal with increment 7
deal into new stack
deal into new stack
            "
            .trim()
        ),
        vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]
    );

    assert_eq!(
        shuffle_cards(
            (0..10).collect(),
            r"
cut 6
deal with increment 7
deal into new stack
            "
            .trim()
        ),
        vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]
    );

    assert_eq!(
        shuffle_cards(
            (0..10).collect(),
            r"
deal with increment 7
deal with increment 9
cut -2
            "
            .trim()
        ),
        vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]
    );

    assert_eq!(
        shuffle_cards(
            (0..10).collect(),
            r"
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
            "
            .trim()
        ),
        vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]
    );
}
