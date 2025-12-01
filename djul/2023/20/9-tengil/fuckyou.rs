#!/usr/bin/env rust-script


use std::io::{stdin, stdout, Write};

fn main() {
    let words: Vec<_> = stdin().lines().map(|l| l.unwrap().into_bytes()).collect();
    let items = words.len();
    assert_eq!(items, 8);
    let mut out = stdout().lock();
    let max = words.iter().map(|l| l.len()).max().unwrap();
    let mut buffer = vec![0; items * max + 10];
    for n0 in 0..items {
        let l0 = (words[n0].len());
        buffer[0..l0].copy_from_slice(&words[n0]);
        for n1 in 0..items {
            let l1 = l0 + words[n1].len();
            buffer[l0..l1].copy_from_slice(&words[n1]);
            for n2 in 0..items {
                let l2 = l1 + words[n2].len();
                buffer[l1..l2].copy_from_slice(&words[n2]);
                for n3 in 0..items {
                    let l3 = l2 + words[n3].len();
                    buffer[l2..l3].copy_from_slice(&words[n3]);
                    for n4 in 0..items {
                        let l4 = l3 + words[n4].len();
                        buffer[l3..l4].copy_from_slice(&words[n4]);
                        for n5 in 0..items {
                            let l5 = l4 + words[n5].len();
                            buffer[l4..l5].copy_from_slice(&words[n5]);
                            for n6 in 0..items {
                                let l6 = l5 + words[n6].len();
                                buffer[l5..l6].copy_from_slice(&words[n6]);
                                for n7 in 0..items {
                                    let l7 = l6 + words[n7].len();
                                    buffer[l6..l7].copy_from_slice(&words[n7]);
                                    buffer[l7] = b'\n';
                                    out.write(&buffer[..l7 + 1]).unwrap();
                                }
                            }
                        }
                    }
                }
            }
            eprintln!("{n0}.{n1}...");
        }
    }
}
