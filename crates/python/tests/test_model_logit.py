from neofoodclub import NeoFoodClub


def test_mer_bets_With_bet_amouunt(
    nfc_with_bet_amount_logit_model: NeoFoodClub,
) -> None:
    bets = nfc_with_bet_amount_logit_model.make_max_ter_bets()

    # the exact bet identities depend on the trained logit coefficients,
    # which are retrained monthly, so only assert on properties that don't
    # change when the coefficients do
    assert len(bets.binaries) == 10
    assert len(set(bets.binaries)) == 10
