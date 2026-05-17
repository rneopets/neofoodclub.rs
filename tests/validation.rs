use neofoodclub::nfc::NeoFoodClub;
use neofoodclub::round_data::RoundData;

fn get_base_round_data() -> RoundData {
    RoundData {
        round: 9300,
        pirates: [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
            [17, 18, 19, 20],
        ],
        currentOdds: [
            [1, 2, 3, 4, 5],
            [1, 6, 7, 8, 9],
            [1, 2, 3, 4, 5],
            [1, 6, 7, 8, 9],
            [1, 2, 3, 4, 5],
        ],
        openingOdds: [
            [1, 2, 3, 4, 5],
            [1, 6, 7, 8, 9],
            [1, 2, 3, 4, 5],
            [1, 6, 7, 8, 9],
            [1, 2, 3, 4, 5],
        ],
        foods: None,
        winners: None,
        customOdds: None,
        start: None,
        timestamp: None,
        changes: None,
        lastChange: None,
    }
}

#[test]
fn test_round_zero() {
    let mut data = get_base_round_data();
    data.round = 0;
    let result = NeoFoodClub::new(data, None, None, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Round number must be greater than 0."));
}

#[test]
fn test_duplicate_pirates() {
    let mut data = get_base_round_data();
    data.pirates[0][0] = 5;
    let result = NeoFoodClub::new(data, None, None, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Pirates must be unique."));
}

#[test]
fn test_invalid_pirate_id() {
    let mut data = get_base_round_data();
    data.pirates[0][0] = 21;
    let result = NeoFoodClub::new(data, None, None, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Pirate IDs must be between 1 and 20."));
}

#[test]
fn test_current_odds_first_element_not_1() {
    let mut data = get_base_round_data();
    data.currentOdds[0][0] = 2;
    let result = NeoFoodClub::new(data, None, None, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("First integer in each arena in currentOdds must be 1."));
}

#[test]
fn test_current_odds_out_of_range() {
    let mut data = get_base_round_data();
    data.currentOdds[0][1] = 14;
    let result = NeoFoodClub::new(data, None, None, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Odds must be between 2 and 13."));
}

#[test]
fn test_opening_odds_first_element_not_1() {
    let mut data = get_base_round_data();
    data.openingOdds[0][0] = 2;
    let result = NeoFoodClub::new(data, None, None, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("First integer in each arena in openingOdds must be 1."));
}

#[test]
fn test_opening_odds_out_of_range() {
    let mut data = get_base_round_data();
    data.openingOdds[0][1] = 1;
    let result = NeoFoodClub::new(data, None, None, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Odds must be between 2 and 13."));
}

#[test]
fn test_invalid_food_id() {
    let mut data = get_base_round_data();
    data.foods = Some([[41; 10]; 5]);
    let result = NeoFoodClub::new(data, None, None, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Food integers must be between 1 and 40."));
}

#[test]
fn test_invalid_winners() {
    let mut data = get_base_round_data();
    data.winners = Some([1, 2, 3, 4, 0]);
    let result = NeoFoodClub::new(data, None, None, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Winners must either be all 0, or all 1-4."));
}
