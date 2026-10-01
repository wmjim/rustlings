// 这个练习探讨 `Cow`（Clone-On-Write，写时克隆）智能指针。
// 它可以封装借用的数据并提供不可变访问，并在需要修改或需要所有权时
// 惰性地克隆数据。这个类型通过 `Borrow` trait 来适配一般的借用数据。

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // 如果数据还不是自己拥有的，就克隆一份到向量里。
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // 因为 `input` 需要被修改，所以发生了克隆。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // 因为 `input` 不需要被修改，所以没有发生克隆。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO: 把 `todo!()` 替换成 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_no_mutation() {
        // 我们也可以不传 `&vec`，这样 `Cow` 就直接拥有它。
        // 这种情况下不会发生修改（所有数字已经是绝对值），因此也不会发生克隆。
        // 但结果仍然归 `Cow` 所有，因为它从未被借用或修改过。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: 把 `todo!()` 替换成 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_mutation() {
        // 当然，如果确实发生了修改（并非所有数字都是绝对值），情况也是如此。
        // 这时 `abs_all` 中对 `to_mut()` 的调用返回的仍然是对原来那份数据的引用。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: 把 `todo!()` 替换成 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }
}
