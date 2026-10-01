// 这个函数返回冰箱里还剩多少冰淇淋。
// 如果在 22:00（24 小时制）之前，还剩 5 勺。到了 22:00，
// 有人把它全吃光了，所以一点冰淇淋都不剩（值为 0）。
// 如果 `hour_of_day` 大于 23，则返回 `None`。
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: 补全函数体。
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: 修复这个测试。怎样才能取出 Option 中装着的值？
        let ice_creams = maybe_ice_cream(12);

        assert_eq!(ice_creams, 5); // 不要修改这一行。
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}
