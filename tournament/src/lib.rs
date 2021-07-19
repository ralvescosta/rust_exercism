use std::collections::HashMap;

#[derive(Default)]
struct Team {
    Name: String,
    MatchPoint: u32,
    Win: u32,
    Draw: u32,
    Loss: u32,
    Point: u32,
}

impl Team {
    pub fn new(name: String) -> Self {
        Self {
            Name: name,
            MatchPoint: 0,
            Win: 0,
            Draw: 0,
            Loss: 0,
            Point: 0,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let matches = match_results.split("\n");
    let mut teams: Vec<Team> = vec![];
    for m in matches {
        let split: Vec<&str> = m.split(";").collect();
        let home_team = split[0].to_string();
        let visiting_team = split[1].to_string();
        let match_result = split[2];

        if let Some(_) = teams.iter().find(|&t| t.Name == home_team) {
        } else {
            teams.push(Team::new(home_team.clone()))
        }

        if let Some(_) = teams.iter().find(|&t| t.Name == visiting_team) {
        } else {
            teams.push(Team::new(visiting_team.clone()))
        }

        match match_result {
            "win" => {
                for team in teams.iter_mut() {
                    if team.Name == home_team {
                        team.MatchPoint += 1;
                        team.Win += 1;
                        team.Point += 3;
                    }

                    if team.Name == visiting_team {
                        team.MatchPoint += 1;
                        team.Loss += 1;
                    }
                }
            }
            "loss" => {
                for team in teams.iter_mut() {
                    if team.Name == visiting_team {
                        team.MatchPoint += 1;
                        team.Win += 1;
                        team.Point += 3;
                    }

                    if team.Name == home_team {
                        team.MatchPoint += 1;
                        team.Loss += 1;
                    }
                }
            }
            "draw" => {
                for team in teams.iter_mut() {
                    if team.Name == home_team || team.Name == visiting_team {
                        team.MatchPoint += 1;
                        team.Draw += 1;
                        team.Point += 1;
                    }
                }
            }
            _ => (),
        }
    }

    teams.sort_by(|f, s| f.Name[..1].cmp(&s.Name[..1]));
    teams.sort_by(|f, s| s.Point.cmp(&f.Point));

    String::from("")
}
