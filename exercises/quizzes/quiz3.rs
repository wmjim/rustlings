// 这个测验考查：
// - 泛型
// - Trait
//
// 一所虚构的魔法学校用 Rust 写了一套新的成绩单生成系统！
// 目前，这套系统只支持生成以数字表示成绩的成绩单（例如 1.0 -> 5.5）。
// 但学校也会给出字母成绩（A+ -> F-），所以系统必须能打印这两种成绩单！
//
// 请在 `ReportCard` 结构体和它的 impl 块中做出必要的修改，
// 让它除了数字成绩单之外也支持字母成绩单。

// TODO: 按上面的描述调整这个结构体。
struct ReportCard {
    grade: f32,
    student_name: String,
    student_age: u8,
}

// TODO: 按上面的描述调整这个 impl 块。
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
