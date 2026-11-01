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
    -0.5992834256604935,
    -2.37467364344103,
    -3.485402512215816,
    -1.5096573103501767,
    -1.8203995746384434,
    -2.4645429778888746,
    -2.265665302800034,
    -2.934555524200841,
    -3.8514357272391466,
    -3.5740459539566305,
    -3.1916224338959767,
    -2.3959443085593595,
    -1.736653675091884,
    -2.5144925600542165,
    0.0,
    -1.2918375100998827,
    -1.1258147730421817,
    -2.2638625515635162,
    -0.5792917591340841,
    -1.5834021828796423,
];
static LOGIT_PFA: [f64; 20] = [
    0.15504049828704225,
    0.2506728770619722,
    0.23143457748354573,
    0.17598253572447578,
    0.25242607456256866,
    0.3028754258122658,
    0.2388412695702957,
    0.27128084635195004,
    0.34292097477022215,
    0.19541024210717783,
    0.1637393134719121,
    0.2291058952563766,
    0.23620167390505917,
    0.2448603872096471,
    0.26829201173487444,
    0.18986971183706475,
    0.15852716887726997,
    0.18561287048458638,
    0.2625969997070659,
    0.2861163167533442,
];
static LOGIT_NFA: [f64; 20] = [
    0.4710187021479001,
    0.31702633653481,
    0.2812758744413969,
    0.5045351710948788,
    0.3837153583838977,
    0.3956604449675084,
    0.3150361599911379,
    0.3228072905808606,
    0.23294572038844114,
    0.330827781422598,
    0.38846725803190946,
    0.4527763442943702,
    0.46715102742085707,
    0.38682553655589424,
    0.4941390447447756,
    0.44015723125701756,
    0.48322777471175227,
    0.45591998553492163,
    0.419464913433055,
    0.3751616496415197,
];
static LOGIT_IS_POS2: [f64; 20] = [
    0.054533644593721134,
    0.018804261640506303,
    0.2373065716318377,
    0.3076062948856534,
    0.2139356959956101,
    0.17780368486130327,
    0.31147354210245726,
    0.12257020848375826,
    0.12594172218922714,
    0.5438184793788553,
    0.5859686221634296,
    0.3091538209223381,
    0.3857857936675839,
    0.1827664623497356,
    0.15254317703257642,
    0.09542897337673757,
    0.06915434277189907,
    0.4218650337283167,
    0.21348772545582834,
    0.08758991860022668,
];
static LOGIT_IS_POS3: [f64; 20] = [
    0.3633131030534103,
    0.40526387764213934,
    0.6419875281836155,
    0.613725274130383,
    0.4743547083341199,
    0.3863006445340219,
    0.5383045894284851,
    0.2794021586149515,
    0.5244732047610545,
    0.8371079378552194,
    0.6202208259519533,
    0.6137124031279493,
    0.5559642034875464,
    0.44770536928853877,
    0.4011039907841231,
    0.39741338583502384,
    0.24554854243046897,
    0.6435295734025138,
    0.5150409191865268,
    0.49871282828210045,
];
static LOGIT_IS_POS4: [f64; 20] = [
    0.5718582567412034,
    0.6198455409098985,
    0.8446909666413814,
    0.897388796959294,
    0.7844854802902563,
    0.6720170131130181,
    0.8216236341119872,
    0.6785400285315856,
    0.9454732350206098,
    1.0580962993381213,
    1.089487852516983,
    0.9731050232552966,
    0.9527059657329325,
    0.6900614864342802,
    0.5521349027861138,
    0.7418870562380944,
    0.544132875424399,
    0.9582540173228871,
    0.6865992861097884,
    0.7508923048750705,
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
