use quake_log_parser::{DeathCause, DeathReport, LogParser, Summary};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
};


#[test]
fn test_parser() {
    let file = File::open("tests/test.log").unwrap();
    let parser = LogParser::new(file);

    let dono_da_bola = "Dono da Bola".to_string();
    let mocinha = "Mocinha".to_string();
    let isgalamido = "Isgalamido".to_string();
    let zeh = "Zeh".to_string();
    let mal = "Mal".to_string();
    let assasinu_credi = "Assasinu Credi".to_string();
    let ootsimo = "Oootsimo".to_string();

    let game2 = Summary {
        total_kills: 11,
        players: HashSet::from_iter(vec![&mocinha, &isgalamido].into_iter()),
        kills: HashMap::from_iter(vec![(&mocinha, 0), (&isgalamido, -9)].into_iter()),
        death_report: None,
    };

    let game21 = Summary {
        total_kills: 131,
        players: HashSet::from_iter(
            vec![
                &dono_da_bola,
                &zeh,
                &isgalamido,
                &mal,
                &assasinu_credi,
                &ootsimo,
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
        death_report: None,
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
    let mut game_count = 0;
    for game in parser {
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
        game_count += 1;
    }
    assert_eq!(game_count, 21);
}
