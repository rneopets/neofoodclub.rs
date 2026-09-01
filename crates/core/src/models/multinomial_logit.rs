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
    -0.5922563645843367,
    -2.3425237213807035,
    -3.4789784966351047,
    -1.4715548015399544,
    -1.8265396719725544,
    -2.436247591722442,
    -2.287403824592425,
    -2.9291155059160228,
    -3.84506117248464,
    -3.5527630488399877,
    -3.194977387686391,
    -2.415575359828276,
    -1.7521945015245226,
    -2.5121158413508304,
    0.0,
    -1.2838131683611516,
    -1.0967571455292775,
    -2.2582014257827625,
    -0.5815161932950952,
    -1.5813323582852825,
];
static LOGIT_PFA: [f64; 20] = [
    0.15407781332968026,
    0.24830101732465465,
    0.23124060711838065,
    0.17421009566267032,
    0.259670189955822,
    0.3006440056125414,
    0.2386050541904779,
    0.2715346800245361,
    0.33439074285831827,
    0.19252934634482666,
    0.16278688712112105,
    0.23393779556501762,
    0.23520516470458866,
    0.23990820471939298,
    0.2677057673129447,
    0.18600190428937327,
    0.15542425911451685,
    0.18212171798881618,
    0.258937943961985,
    0.2863810166878703,
];
static LOGIT_NFA: [f64; 20] = [
    0.47147900878666915,
    0.324385928492838,
    0.28254170773743265,
    0.5193428717621762,
    0.3813013770304785,
    0.3969201396429939,
    0.31146395354808576,
    0.32065284970166025,
    0.23380454539340467,
    0.32829702757539936,
    0.3844915050001845,
    0.4576130286413404,
    0.4681711895558233,
    0.3824864634756385,
    0.49522237249628276,
    0.4406830295691128,
    0.48043690055188615,
    0.46169083170516584,
    0.4192876757370015,
    0.36953558399640735,
];
static LOGIT_IS_POS2: [f64; 20] = [
    0.05660836583983094,
    0.02433911367022194,
    0.23980344143360235,
    0.2949583076622342,
    0.21630935033036336,
    0.15808889811560498,
    0.3475434636880345,
    0.1149435991851947,
    0.1527423912316273,
    0.5250274611186194,
    0.5957630313336879,
    0.3362773255436444,
    0.41867644980210045,
    0.198738378303865,
    0.15513983307240448,
    0.1061453706022262,
    0.05438993739998229,
    0.4287706144981264,
    0.22841631150738045,
    0.09280232790936474,
];
static LOGIT_IS_POS3: [f64; 20] = [
    0.3594203379389299,
    0.3851630469710927,
    0.6336391258530353,
    0.5993141947429397,
    0.47076113234287004,
    0.344194217695785,
    0.5506348330323461,
    0.27779707147459987,
    0.5478805073662738,
    0.8237040413982726,
    0.6360075851556718,
    0.6314958815106855,
    0.5803269296180349,
    0.4643909683735395,
    0.4182073405094401,
    0.40018191329988506,
    0.21824892804405668,
    0.6584189100149522,
    0.5225526642238697,
    0.49240834334389527,
];
static LOGIT_IS_POS4: [f64; 20] = [
    0.5705516923208811,
    0.6252495424047732,
    0.855699145023031,
    0.8809619259529189,
    0.778119058933689,
    0.6459115816275228,
    0.8391537530826562,
    0.6672016648360096,
    0.9449632111857171,
    1.0433912265034062,
    1.0885666067240365,
    0.9937359938501135,
    0.9849400464101814,
    0.7077510308077146,
    0.5672837665098934,
    0.7503405941957411,
    0.5258579676754885,
    0.9761636012761364,
    0.7024452077086765,
    0.7547589013767317,
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
