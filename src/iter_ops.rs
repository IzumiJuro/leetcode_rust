#[allow(dead_code)]
fn quick_sort(v: &mut [i32]) {
    if v.len() <= 1 {
        return;
    }

    let p = v[0];
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = v[1..].iter().partition(|e| **e < p);
    quick_sort(&mut a);
    quick_sort(&mut b);
    v[..a.len()].copy_from_slice(&a);
    v[a.len()] = p;
    v[a.len() + 1..].copy_from_slice(&b);
}

#[cfg(test)]
mod test {
    #[test]
    fn iter_ops_1_works() {
        let v = vec![
            String::from("hello"),
            String::from("world"),
            String::from("hi"),
        ];
        let mut it = v.iter();
        assert_eq!(it.next(), Some(&String::from("hello")));
        assert_eq!(it.next(), Some(&String::from("world")));
        assert_eq!(it.next(), Some(&String::from("hi")));
        assert_eq!(it.next(), None);
    }
    #[test]
    fn iter_ops_2_works() {
        let v = vec!["one", "two", "three"];
        v.iter().for_each(|&e| println!("{e}"))
    }

    #[test]
    fn iter_ops_3_works() {
        // collect
        let v: Vec<_> = (0..10).collect();

        // sum, product
        assert_eq!(v.iter().sum::<i32>(), 45);
        assert_eq!(v.iter().product::<i32>(), 0);

        // fold, reduce , rfold, try_xxx
        assert_eq!(
            v.iter().fold(0, |acc, e| {
                println!("{acc} + {e}");
                acc + e
            }),
            45
        );
        println!();
        assert_eq!(
            v.iter().rev().fold(0, |acc, e| {
                println!("{acc} + {e}");
                acc + e
            }),
            45
        );
        println!();
        assert_eq!(
            v.iter().rfold(0, |acc, e| {
                println!("{acc} + {e}");
                acc + e
            }),
            45
        );
        println!();
        assert_eq!(
            v.iter().rfold(1, |acc, e| {
                println!("{acc} * {e}");
                acc * e
            }),
            0
        );
        println!();
        assert_eq!(
            v.iter().copied().reduce(|acc, e| {
                println!("{acc} * {e}");
                acc * e
            }),
            Some(0)
        );
        // max min
        assert_eq!(v.iter().max(), Some(&9));
        assert_eq!(v.iter().min(), Some(&0));

        assert_eq!(
            v.iter().reduce(|cur_max, cur| cur_max.max(cur)),
            v.iter().max()
        );

        // all any
        assert!(v.iter().all(|e| *e < 10));
        assert!(v.iter().any(|e| *e == 5));

        // max_by, min_by, max_by_key, min_by_key
        #[allow(dead_code)]
        #[derive(Debug)]
        struct S {
            key: i32,
            value: &'static str,
        }
        let v = [
            S {
                key: 9,
                value: "hello",
            },
            S {
                key: 12,
                value: "world",
            },
            S {
                key: 5,
                value: "wqejo",
            },
        ];
        println!("{:?}", v.iter().max_by(|&a, &b| a.key.cmp(&b.key)));
        println!("{:?}", v.iter().min_by(|&a, &b| a.key.cmp(&b.key)));
        println!("{:?}", v.iter().max_by_key(|a| a.key));
        println!("{:?}", v.iter().min_by_key(|a| a.key))
    }

    #[test]
    fn iter_ops_4_works() {
        let v = [1_u8, 2, u8::MAX, 3, 4];

        // skip, skip_while
        println!("{:?}", v.iter().skip(2).collect::<Vec<_>>());
        println!(
            "{:?}",
            v.iter().skip_while(|e| **e != u8::MAX).collect::<Vec<_>>()
        );
        // take, take_while
        println!("{:?}", v.iter().take(2).collect::<Vec<_>>());
        println!(
            "{:?}",
            v.iter().take_while(|e| **e != u8::MAX).collect::<Vec<_>>()
        );
        // map, map_while
        println!("{:?}", v.iter().map(|e| e - 1).collect::<Vec<_>>());
        println!(
            "{:?}",
            v.iter()
                .map_while(|e| 1_u8.checked_add(*e))
                .collect::<Vec<_>>()
        );
        // flatten, flat_map
        let w = [Some(0), None, Some(1)];
        println!("{:?}", w.iter().flatten().collect::<Vec<_>>());
        println!(
            "{:?}",
            v.iter()
                .flat_map(|e| e.checked_add(1_u8))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn iter_ops_5_works() {
        let mut v = [3, 5, 7, 2, 2, 1, 7, 2];

        v.sort_unstable();

        let it1 = v[1..].iter();
        let it2 = v[..v.len() - 1].iter();
        let mut r = it1
            .zip(it2)
            .filter_map(|(a, b)| (a == b).then_some(b))
            .collect::<Vec<_>>();
        println!("{:?}", r);
        // remove duplication
        r.dedup();
        println!("{:?}", r);
    }

    #[test]
    fn iter_ops_6_works() {
        use super::quick_sort;
        let mut v = [3, 5, 7, 2, 2, 1, 7, 2];

        // let (a, b): (Vec<i32>, Vec<i32>) = v.iter().partition(|e| **e < 5);
        // println!("{:?} {:?}", a, b);
        quick_sort(&mut v);
        println!("{:?}", v);
    }

    #[test]
    fn iter_ops_7_works() {
        let v = [3, 5, 7, 2, 2, 1, 7, 2];

        // position, rposition
        assert_eq!(Some(2), v.iter().position(|e| *e == 7));
        assert_eq!(Some(&7), v.iter().find(|e| **e == 7));
        assert_eq!(Some(-1), v.iter().find_map(|e| (*e == 7).then_some(-1)));
    }

    #[test]
    fn iter_ops_8_works() {
        let v = [1, 2, 3];
        let u = [4, 5, 6];
        // python itertools

        // cycle
        for i in v.iter().cycle().take(20) {
            println!("{i}")
        }
        println!();

        // chain
        for i in v.iter().chain(u.iter()) {
            println!("{i}")
        }

        // enumerate
        for i in v.iter().enumerate() {
            println!("{i:?}")
        }

        // inspect
        let v: Vec<_> = v
            .iter()
            .inspect(|&e| {
                println!("{e}");
            })
            .map(Some)
            .inspect(|&e| {
                println!("{e:?}");
            })
            .collect();
        println!("{v:?}")
    }
}
