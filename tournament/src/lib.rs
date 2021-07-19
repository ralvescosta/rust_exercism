#[derive(Default)]
struct Team {
    name: String,
    match_point: u32,
    win: u32,
    draw: u32,
    loss: u32,
    point: u32,
}

impl Team {
    pub fn new_winner(name: String) -> Self {
        Self {
            name,
            match_point: 1,
            win: 1,
            draw: 0,
            loss: 0,
            point: 3,
        }
    }

    pub fn new_looser(name: String) -> Self {
        Self {
            name,
            match_point: 1,
            win: 0,
            draw: 0,
            loss: 1,
            point: 0,
        }
    }
    pub fn new_drawer(name: String) -> Self {
        Self {
            name,
            match_point: 1,
            win: 0,
            draw: 1,
            loss: 0,
            point: 1,
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

        match match_result {
            "win" => winner_score(&mut teams, home_team, visiting_team),
            "loss" => winner_score(&mut teams, visiting_team, home_team),
            "draw" => draw_score(&mut teams, home_team, visiting_team),
            _ => (),
        }
    }

    teams.sort_by(|f, s| f.name[..1].cmp(&s.name[..1]));
    teams.sort_by(|f, s| s.point.cmp(&f.point));

    let mut result = "".to_string() + "Team                           | MP |  W |  D |  L |  P\n";

    String::from("")
}

fn winner_score(teams: &mut Vec<Team>, winner: String, looser: String) {
    let mut has_winner = false;
    let mut has_looser = false;
    for team in teams.iter_mut() {
        if team.name == winner {
            team.match_point += 1;
            team.win += 1;
            team.point += 3;
            has_winner = true;
        }

        if team.name == looser {
            team.match_point += 1;
            team.loss += 1;
            has_looser = true;
        }

        if has_winner && has_looser {
            break;
        }
    }

    if !has_winner {
        teams.push(Team::new_winner(winner))
    }
    if !has_looser {
        teams.push(Team::new_looser(looser))
    }
}

fn draw_score(teams: &mut Vec<Team>, home: String, visiting: String) {
    let mut has_home = false;
    let mut has_visiting = false;
    for team in teams.iter_mut() {
        if team.name == home {
            team.match_point += 1;
            team.draw += 1;
            team.point += 1;
            has_home = true;
        }

        if team.name == visiting {
            team.match_point += 1;
            team.draw += 1;
            team.point += 1;
            has_visiting = false;
        }

        if has_home && has_visiting {
            break;
        }
    }
    if !has_home {
        teams.push(Team::new_drawer(home))
    }
    if !has_visiting {
        teams.push(Team::new_drawer(visiting))
    }
}
