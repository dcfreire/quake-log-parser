use quake_log_parser::{parse_games, DeathCause, DeathReport, Player, Summary};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

#[test]
fn test_parse_games() {
    let file = File::open("tests/test.log").unwrap();
    let games = parse_games(file).unwrap();

    let dono_da_bola = Player::Some("Dono da Bola".to_string());
    let mocinha = Player::Some("Mocinha".to_string());
    let isgalamido = Player::Some("Isgalamido".to_string());
    let zeh = Player::Some("Zeh".to_string());
    let mal = Player::Some("Mal".to_string());
    let assasinu_credi = Player::Some("Assasinu Credi".to_string());
    let ootsimo = Player::Some("Oootsimo".to_string());

    let game2 = Summary {
        total_kills: 11,
        players: &HashSet::from_iter(
            vec![dono_da_bola.clone(), mocinha.clone(), isgalamido.clone()].into_iter(),
        ),
        kills: HashMap::from_iter(
            vec![(&dono_da_bola, 0), (&isgalamido, -9), (&mocinha, 0)].into_iter(),
        ),
        death_report: None
    };

    let game21 = Summary {
        total_kills: 131,
        players: &HashSet::from_iter(
            vec![
                dono_da_bola.clone(),
                zeh.clone(),
                isgalamido.clone(),
                mal.clone(),
                assasinu_credi.clone(),
                ootsimo.clone(),
            ]
            .into_iter(),
        ),
        kills: HashMap::from_iter(
            vec![
                (&dono_da_bola, 10),
                (&isgalamido, 17),
                (&ootsimo, 20),
                (&zeh, 19),
                (&mal, 6),
                (&assasinu_credi, 13),
            ]
            .into_iter(),
        ),
        death_report: None
    };

    let game21_report = DeathReport {
        report: HashMap::from_iter(vec![
            (DeathCause::RocketSplash, 60),
            (DeathCause::Rocket, 37),
            (DeathCause::Machinegun, 4),
            (DeathCause::Shotgun, 4),
            (DeathCause::Falling, 3),
            (DeathCause::TriggerHurt, 14),
            (DeathCause::Railgun, 9),
        ]),
    };

    let game2_report = DeathReport {
        report: HashMap::from_iter(vec![
            (DeathCause::RocketSplash, 3),
            (DeathCause::Falling, 1),
            (DeathCause::TriggerHurt, 7),
        ]),
    };
    assert_eq!(games.len(), 21);
    for game in games {
        let summary = game.match_summary(false);
        let death_report = game.death_report();
        if game.id == 2 {
            assert_eq!(summary, game2);
            assert_eq!(death_report, game2_report);
        } else if game.id == 21 {
            assert_eq!(summary, game21);
            assert_eq!(death_report, game21_report);
        } else {
            //println!("game_{}: {}", game.id, game.match_summary())
            //println!("game_{}: {}", game.id, &game.death_report());
        }
        assert_eq!(
            death_report.report.values().sum::<isize>(),
            summary.total_kills as isize
        );
    }
}
