from neofoodclub import NeoFoodClub


def test_is_same_as_true_same_order(nfc: NeoFoodClub) -> None:
    a = nfc.make_bets_from_binaries([0x1, 0x2, 0x4])
    b = nfc.make_bets_from_binaries([0x1, 0x2, 0x4])
    assert a.is_same_as(b) is True


def test_is_same_as_true_shuffled_order(nfc: NeoFoodClub) -> None:
    a = nfc.make_bets_from_binaries([0x1, 0x2, 0x4])
    b = nfc.make_bets_from_binaries([0x4, 0x1, 0x2])
    assert a.is_same_as(b) is True


def test_is_same_as_false_different_bet(nfc: NeoFoodClub) -> None:
    a = nfc.make_bets_from_binaries([0x1, 0x2, 0x4])
    b = nfc.make_bets_from_binaries([0x1, 0x2, 0x8])
    assert a.is_same_as(b) is False


def test_is_same_as_false_duplicate_bet(nfc: NeoFoodClub) -> None:
    a = nfc.make_bets_from_binaries([0x1, 0x2])
    b = nfc.make_bets_from_binaries([0x1, 0x2, 0x2])
    assert a.is_same_as(b) is False


def test_is_same_as_true_empty(nfc: NeoFoodClub) -> None:
    a = nfc.make_bets_from_binaries([])
    b = nfc.make_bets_from_binaries([])
    assert a.is_same_as(b) is True


def test_is_same_as_ignores_amounts(nfc: NeoFoodClub) -> None:
    a = nfc.make_bets_from_binaries([0x1, 0x2])
    b = nfc.make_bets_from_binaries([0x1, 0x2])
    b.set_amounts_with_int(99)
    assert a.is_same_as(b) is True


def test_identity_sorted(nfc: NeoFoodClub) -> None:
    a = nfc.make_bets_from_binaries([0x4, 0x1, 0x2])
    b = nfc.make_bets_from_binaries([0x1, 0x2, 0x4])
    assert a.identity() == b.identity()
    assert a.identity() == sorted(a.identity())


def test_identity_ignores_amounts(nfc: NeoFoodClub) -> None:
    a = nfc.make_bets_from_binaries([0x1, 0x2])
    b = nfc.make_bets_from_binaries([0x1, 0x2])
    b.set_amounts_with_int(99)
    assert a.identity() == b.identity()


def test_identity_empty(nfc: NeoFoodClub) -> None:
    a = nfc.make_bets_from_binaries([])
    assert a.identity() == []
