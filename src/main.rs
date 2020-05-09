use std::cmp::Ordering;
use rand::Rng;

struct Player {
    skill: u8,
    name: String,
    rank: i32,
    total_wins: i32,
}

fn create_player(skill: u8) -> Player {
    let mut name = String::new();
    name.push_str("Player-");
    name.push_str(&skill.to_string());
    name.push_str("-");
    Player {
        skill,
        rank: 1000,
        name: name,
        total_wins: 0,
    }
}

fn create_playerlist(number:u8) -> Vec<Player> {
    let mut playerlist = Vec::with_capacity(usize::from(number));
    for x in 0..number {
        playerlist.push(create_player(x));
    }
    return playerlist
}

fn play_match(mut playerlist:Vec<Player>, p1:usize, p2:usize) -> Vec<Player> {
    println!("{}, {}, {} VS {}, {}, {}", &playerlist[p1].name, &playerlist[p1].skill, &playerlist[p1].rank, &playerlist[p2].name, &playerlist[p2].skill, &playerlist[p2].rank);
    let base:i32 = 10;
    let transformed_rating_p1:f64 = base.pow(playerlist[p1].rank as u32 / 400) as f64;
    let transformed_rating_p2:f64 = base.pow(playerlist[p2].rank as u32 / 400) as f64;
    let expected_score_p1:f64 = transformed_rating_p1 / (transformed_rating_p1 + transformed_rating_p2);
    let expected_score_p2:f64 = transformed_rating_p2 / (transformed_rating_p2 + transformed_rating_p1);
    let score_p1:f64;
    let score_p2:f64;
    match playerlist[p1].skill.cmp(&playerlist[p2].skill) {
        Ordering::Less => {
            println!("p2 win");
            score_p1 = 0.0;
            score_p2 = 1.0;
            playerlist[p2].total_wins = playerlist[p2].total_wins + 1;
        },
        Ordering::Greater => {
            println!("p1 win");
            score_p1 = 1.0;
            score_p2 = 0.0;
            playerlist[p1].total_wins = playerlist[p1].total_wins + 1;
        },
        Ordering::Equal => {
            println!("tie!");
            score_p1 = 0.5;
            score_p2 = 0.5;
        },
    }

    playerlist[p1].rank = (f64::from(playerlist[p1].rank) + 32.0 * (score_p1 - expected_score_p1)).round() as i32;
    playerlist[p2].rank = (f64::from(playerlist[p2].rank) + 32.0 * (score_p2 - expected_score_p2)).round() as i32;
    return playerlist
}

fn play_all_matches(mut playerlist:Vec<Player>) -> Vec<Player> {
    const STEP_SIZE:usize = 10;
    let num_players = playerlist.len();
    if num_players % STEP_SIZE != 0 {
        panic!("NUM_PLAYERS not divisible by STEP_SIZE");
    }

    let mut match_list: Vec<_> = (0..num_players).collect();
    match_list = knuth_shuffle(match_list);

    for offset in (0..num_players).step_by(10) {
        for j in offset..9 + offset {
            for k in j+1..10 + offset{
                playerlist = play_match(playerlist, match_list[j], match_list[k]);
            }
        }
    }
    return playerlist
}

fn knuth_shuffle(mut vector:Vec<usize>) -> Vec<usize> {
    let mut holder:usize;
    let n = vector.len();
    for i in 0..n-1 {
        let rand_num = rand::thread_rng().gen_range(i, n);
        holder = vector[i];
        vector[i] = vector[rand_num];
        vector[rand_num] = holder;
    }
    return vector
}


fn main() {
    const NUM_PLAYERS:u8 = 100;
    println!("start");
    println!("Creating Players");
    let mut playerlist = create_playerlist(NUM_PLAYERS);
    if NUM_PLAYERS % 10 != 0 {
        println!("Bad number of players");
        return
    }
    println!("Name, Skill, Rank");
    for player in &playerlist {
        println!("{}, {}, {}", &player.name, &player.skill, &player.rank);
    }
    println!("Created Players");
    println!("Playing");
    for _x in 0..20 {
        playerlist = play_all_matches(playerlist);
        println!("Name, Skill, Rank, Wins");
        for player in &playerlist {
            println!("{}, {}, {}, {}", &player.name, &player.skill, &player.rank, &player.total_wins);
        }
    }
}
