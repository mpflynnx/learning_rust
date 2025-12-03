// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default,Debug)]
pub struct TeamScores {
    pub goals_scored: u8,
    pub goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct TeamScores is the value.
    let mut scores = HashMap::<&str, TeamScores>::new();
    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap();

        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: Populate the scores table with the extracted details.
        // Keep in mind that goals scored by team 1 will be the number of goals
        // conceded by team 2. Similarly, goals scored by team 2 will be the
        // number of goals conceded by team 1.
        
        // Rustlings solution
        // Insert the default with zeros if a team doesn't exist yet.
        let team_1 = scores.entry(team_1_name).or_default(); // team_1 of type @mut scores
        // Update the values.
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;

        // Similarly for the second team.
        let team_2 = scores.entry(team_2_name).or_default();
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;

        // My solution
        //// https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
        //scores
            //.entry(team_1_name)
            //.and_modify(|team_scores| {
                //// This closure runs ONLY if the key (team_1_name) exists.
                //// 'team_scores' is a mutable reference to the TeamScores struct.
                //team_scores.goals_scored += team_1_score;
                //team_scores.goals_conceded += team_2_score;
            //})
            //.or_insert(
                //// This closure/value is only used if the key does NOT exist.
                //// In this case, we'd insert a new team_1_name.
                //TeamScores {
                    //goals_scored: (team_1_score),
                    //goals_conceded: (team_2_score),
                //},
            //);

        //scores
            //.entry(team_2_name)
            //.and_modify(|team_scores| {
                //team_scores.goals_scored += team_2_score;
                //team_scores.goals_conceded += team_1_score;
            //})
            //.or_insert(TeamScores {
                //goals_scored: (team_2_score),
                //goals_conceded: (team_1_score),
            //});
    //}

    }
    scores // mf: implicit return
}

fn main() {
    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    let scores = build_scores_table(RESULTS);

    println!("{:?}", scores);

    // HashMap structure
    // {
    // "Poland": TeamScores { goals_scored: 2, goals_conceded: 0 },
    // "Italy": TeamScores { goals_scored: 1, goals_conceded: 3 },
    // "Spain": TeamScores { goals_scored: 0, goals_conceded: 3 },
    // "Germany": TeamScores { goals_scored: 2, goals_conceded: 1 },
    // "France": TeamScores { goals_scored: 5, goals_conceded: 5 },
    // "England": TeamScores { goals_scored: 6, goals_conceded: 4 }
    // }
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

        assert!(
            ["England", "France", "Germany", "Italy", "Poland", "Spain"]
                .into_iter()
                .all(|team_name| scores.contains_key(team_name))
        );
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

    #[test]
    fn validate_team_score_3() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Italy").unwrap();
        assert_eq!(team.goals_scored, 1);
        assert_eq!(team.goals_conceded, 3);
    }

    #[test]
    fn validate_team_score_4() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Poland").unwrap();
        assert_eq!(team.goals_scored, 2);
        assert_eq!(team.goals_conceded, 0);
    }

    #[test]
    fn validate_team_score_5() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Germany").unwrap();
        assert_eq!(team.goals_scored, 2);
        assert_eq!(team.goals_conceded, 1);
    }

    #[test]
    fn validate_team_score_6() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("France").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 5);
    }
}
