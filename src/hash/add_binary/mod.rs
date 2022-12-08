struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut vec_a = a
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        let mut vec_b = b
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        let len = vec_a.len().max(vec_b.len());
        let mut carry = 0;
        let mut res = Vec::new();

        (0..len).for_each(|_| {
            let a = vec_a.pop().unwrap_or(0);
            let b = vec_b.pop().unwrap_or(0);
            let sum = a + b + carry;
            carry = 0;
            if sum > 1 {
                carry = 1;
                res.push(sum - 2);
            } else {
                res.push(sum);
            }
        });
        if carry == 1 {
            res.push(1);
        }

        res.iter().map(|i| i.to_string()).rev().collect()
    }
}
