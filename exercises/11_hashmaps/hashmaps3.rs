// 给定一份足球比赛的比分列表，每行一场比赛。每行的格式是
// "<队1名称>,<队2名称>,<队1进球数>,<队2进球数>"
// 例如："England,France,4,2"（英格兰进 4 球，法国进 2 球）。
//
// 你需要构建一张积分表，其中包含球队的名称、该队的总进球数，
// 以及该队的总失球数。

use std::collections::HashMap;

// 用于存储某支球队进球情况的结构体。
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // 球队名称是键，与之关联的结构体是值。
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // 注意：这里用 `unwrap` 是因为我们还没有学到错误处理。
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: 用提取出的信息填充积分表。
        // 记住：队 1 的进球数就是队 2 的失球数，队 2 的进球数就是队 1 的失球数。
    }

    scores
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
