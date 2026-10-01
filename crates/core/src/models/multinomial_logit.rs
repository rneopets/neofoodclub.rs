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
    -0.597311488706159,
    -2.3636746940987736,
    -3.497267599100136,
    -1.4895427274508715,
    -1.8170373744833739,
    -2.4464397957546082,
    -2.277788131220391,
    -2.9260820633447597,
    -3.8465443539086808,
    -3.560515721048153,
    -3.205904669936476,
    -2.4124589325770804,
    -1.7553580955467964,
    -2.5090724123163173,
    0.0,
    -1.2839184030692592,
    -1.110568151372929,
    -2.2641820877998926,
    -0.5781939321129945,
    -1.5915032347278812,
];
static LOGIT_PFA: [f64; 20] = [
    0.1529367303767386,
    0.25021922691691356,
    0.23303756105619494,
    0.1740980609390545,
    0.2544258826733096,
    0.30187813236190686,
    0.2390496060767066,
    0.2694404311738083,
    0.344942597831086,
    0.19328431841681842,
    0.16357009685647145,
    0.2333431141856588,
    0.2351593453357466,
    0.2444899175128886,
    0.2671595087313684,
    0.18661230577215962,
    0.15689567116583047,
    0.18334976015702664,
    0.2610044624932983,
    0.28747861743451114,
];
static LOGIT_NFA: [f64; 20] = [
    0.47113224820499133,
    0.3185556534468125,
    0.280734286438733,
    0.5135844291055875,
    0.3818807491944495,
    0.4041715792517398,
    0.31410693834255277,
    0.32466533209880255,
    0.23124663197505702,
    0.33332477356016244,
    0.3791981832509593,
    0.4561546975252918,
    0.4711129883761525,
    0.3823833710741207,
    0.49360624037237993,
    0.4401760217281273,
    0.48077378799232745,
    0.45849886451538757,
    0.4166751446469895,
    0.36934183814784083,
];
static LOGIT_IS_POS2: [f64; 20] = [
    0.05911643802624804,
    0.020722197163698587,
    0.24057437561332937,
    0.29939349169854285,
    0.21351869789434624,
    0.16607210304879744,
    0.3315164123807219,
    0.12674539501432425,
    0.1249360056955304,
    0.5433626435584932,
    0.603946997853376,
    0.32426932501169403,
    0.4196611695782721,
    0.1758968679643108,
    0.15359368958713035,
    0.10369635649812271,
    0.05602533738876166,
    0.4377399188240502,
    0.21596123114057353,
    0.09157519822462916,
];
static LOGIT_IS_POS3: [f64; 20] = [
    0.3678606327585961,
    0.3902545338940641,
    0.6508369602754634,
    0.6080469894254331,
    0.46613450994499017,
    0.3840342937248796,
    0.5379719054048824,
    0.29037674485053816,
    0.524824038663534,
    0.8271805636012315,
    0.6295119781757064,
    0.6240048695449026,
    0.586818079744826,
    0.44621521152191534,
    0.4127882681751522,
    0.3976267527340476,
    0.22658466837566613,
    0.6497992914915632,
    0.5103203027278191,
    0.5018420703737809,
];
static LOGIT_IS_POS4: [f64; 20] = [
    0.5784187697944508,
    0.6251481342235037,
    0.8516970734686915,
    0.8893910430720318,
    0.7830486621623136,
    0.6641884064395401,
    0.8358251676905752,
    0.6775379065606016,
    0.9367405206465402,
    1.0569507996495717,
    1.0906877937523378,
    0.9826542480725023,
    0.9884963561034984,
    0.6854856713976355,
    0.5524299749483452,
    0.7412404192292384,
    0.5363468041088736,
    0.9724499904684885,
    0.689451238072056,
    0.7538138845557174,
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
