use std::f64::consts::E;

use crate::arena::Arenas;

#[derive(Debug, Clone)]
pub struct MultinomialLogitModel;

impl MultinomialLogitModel {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(arenas: &Arenas) -> [[f64; 5]; 5] {
        make_probabilities(arenas)
    }
}

pub fn make_probabilities(arenas: &Arenas) -> [[f64; 5]; 5] {
    let mut probs = [[1.0, 0.0, 0.0, 0.0, 0.0]; 5];

    for arena in &arenas.arenas {
        let mut capabilities = [0.0; 5];
        for pirate in &arena.pirates {
            let pirate_index = pirate.index - 1;
            let pirate_id = pirate.id as usize - 1;
            let mut pirate_strength = LOGIT_INTERCEPTS[pirate_id];
            let favorite = pirate.pfa.unwrap_or(0);
            let allergy = pirate.nfa.unwrap_or(0);
            pirate_strength += LOGIT_PFA[pirate_id] * favorite as f64;
            pirate_strength += LOGIT_NFA[pirate_id] * allergy as f64;

            match pirate_index {
                1 => pirate_strength += LOGIT_IS_POS2[pirate_id],
                2 => pirate_strength += LOGIT_IS_POS3[pirate_id],
                3 => pirate_strength += LOGIT_IS_POS4[pirate_id],
                _ => (),
            }

            capabilities[pirate_index as usize + 1] = E.powf(pirate_strength);
            capabilities[0] += capabilities[pirate_index as usize + 1];
        }

        for pirate in &arena.pirates {
            probs[arena.id as usize][pirate.index as usize] =
                capabilities[pirate.index as usize] / capabilities[0];
        }
    }

    probs
}

// Retrained monthly by automation/final.py (see
// .github/workflows/update-logit-values.yml), which patches this block in
// place. Original methodology: https://github.com/arsdragonfly/neofoodclub

static LOGIT_INTERCEPTS: [f64; 20] = [
    -0.5893848002616592,
    -2.3518020035438556,
    -3.4777331363405484,
    -1.4643810117858584,
    -1.8201318134647224,
    -2.435366640742078,
    -2.287210763647903,
    -2.9323045670109287,
    -3.9094265123730905,
    -3.544151037425662,
    -3.1819259560052093,
    -2.4032578482860183,
    -1.755355266205484,
    -2.512347019015439,
    0.0,
    -1.2743759399031804,
    -1.1025803182199987,
    -2.2620360643055397,
    -0.574682342753546,
    -1.5904470201864838,
];
static LOGIT_PFA: [f64; 20] = [
    0.15340510382954164,
    0.2508890368488035,
    0.2316497054803678,
    0.1719126047040151,
    0.26006468660990495,
    0.30395368896806624,
    0.23751629725991932,
    0.2728791697438482,
    0.3425341504663444,
    0.1919455097066751,
    0.16289916926069278,
    0.23249502401150307,
    0.23616289319995445,
    0.24076558418822458,
    0.26886313342239593,
    0.18626601751436167,
    0.15788563270090358,
    0.1840671560813943,
    0.25633396923983737,
    0.28609390748442975,
];
static LOGIT_NFA: [f64; 20] = [
    0.4691820358071923,
    0.32374433532022734,
    0.279951361185496,
    0.517945959163408,
    0.38217719308366693,
    0.39109744865247614,
    0.31356229806131286,
    0.31735631631860395,
    0.2310483615819262,
    0.33010695942761625,
    0.39276222099232244,
    0.45451242834645866,
    0.46868907363570667,
    0.3788860284249756,
    0.49412753813887367,
    0.4394750476444515,
    0.48318307228756324,
    0.46167899345086305,
    0.421593584773803,
    0.3647454143667999,
];
static LOGIT_IS_POS2: [f64; 20] = [
    0.04873923322522541,
    0.019556780708755116,
    0.24330676640062293,
    0.29342085283596786,
    0.21825957034685198,
    0.1574554444956133,
    0.3622627283489947,
    0.10920660391666848,
    0.21289180129408483,
    0.530569913624752,
    0.5904258899615191,
    0.33211996116553794,
    0.4127496496670264,
    0.1939963894261818,
    0.15389993413097572,
    0.09175442705118297,
    0.060615497698789995,
    0.43179533990708613,
    0.2266906895591633,
    0.10403351740192227,
];
static LOGIT_IS_POS3: [f64; 20] = [
    0.358731483195169,
    0.3846698182684156,
    0.6374634732086467,
    0.5942604022687976,
    0.4693755400751024,
    0.32997190677985816,
    0.5612318731379157,
    0.28003229970386884,
    0.5946666708282198,
    0.8289549658422685,
    0.6328061575259324,
    0.6194083386331689,
    0.5797764638150776,
    0.46901966264129175,
    0.4107744777471082,
    0.3920743255421824,
    0.22737243745879504,
    0.658465562935623,
    0.5261150971543981,
    0.49747500575612597,
];
static LOGIT_IS_POS4: [f64; 20] = [
    0.5643049383930996,
    0.6273412537565366,
    0.8487839310805146,
    0.8823520467867495,
    0.776209008367336,
    0.6304530964786424,
    0.8475667732018569,
    0.6655243796110253,
    0.995873418088202,
    1.04754853816069,
    1.0950692390527965,
    0.9844577810723815,
    0.9838684906030031,
    0.7141023612062847,
    0.5665351475518512,
    0.7384455508668267,
    0.5275151369107098,
    0.9758193252756899,
    0.7076480266514855,
    0.7593570781162101,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::round_data::RoundData;

    const EPSILON: f64 = 1e-9;

    // Real-world fixture round data (round 8765), reused from tests/integration_test.rs.
    const ROUND_DATA_JSON: &str = r#"{"foods":[[5,20,24,21,18,7,34,29,38,8],[26,24,20,36,33,40,5,13,8,25],[5,29,22,31,40,27,30,4,8,19],[35,19,36,5,12,37,6,3,29,30],[28,24,36,17,18,9,1,33,19,3]],"round":8765,"start":"2023-05-05T23:14:57+00:00","changes":null,"pirates":[[6,11,4,3],[14,15,2,9],[10,16,18,20],[1,12,13,5],[8,19,17,7]],"winners":null,"timestamp":"2023-05-06T23:14:20+00:00","lastChange":"2023-05-06T19:21:01+00:00","currentOdds":[[1,11,3,2,3],[1,13,2,7,13],[1,13,2,4,2],[1,2,10,6,6],[1,13,4,2,4]],"customOdds":null,"openingOdds":[[1,11,3,2,4],[1,13,2,5,13],[1,13,2,5,2],[1,2,8,5,5],[1,13,3,2,4]]}"#;

    fn make_arenas() -> Arenas {
        let round_data: RoundData = serde_json::from_str(ROUND_DATA_JSON).unwrap();
        Arenas::new(&round_data)
    }

    #[test]
    fn test_make_probabilities_bounds() {
        let arenas = make_arenas();
        let probs = make_probabilities(&arenas);
        for arena in probs.iter() {
            for &p in arena[1..5].iter() {
                assert!((0.0..=1.0).contains(&p), "probability out of bounds: {p}");
            }
        }
    }

    #[test]
    fn test_make_probabilities_sums_to_one_per_arena() {
        let arenas = make_arenas();
        let probs = make_probabilities(&arenas);
        for arena in probs.iter() {
            let sum: f64 = arena[1..5].iter().sum();
            assert!(
                (sum - 1.0).abs() < EPSILON,
                "arena probabilities did not sum to 1.0: {sum}"
            );
        }
    }

    #[test]
    fn test_multinomial_logit_model_new_matches_make_probabilities() {
        let arenas = make_arenas();
        let from_model = MultinomialLogitModel::new(&arenas);
        let expected = make_probabilities(&arenas);
        assert_eq!(from_model, expected);
    }

    #[test]
    fn test_make_probabilities_first_column_is_always_one() {
        let arenas = make_arenas();
        let probs = make_probabilities(&arenas);
        for arena in probs.iter() {
            assert_eq!(arena[0], 1.0);
        }
    }
}
