from collections.abc import Sequence

import pytest

from neofoodclub import (
    Math,
)


@pytest.mark.parametrize(
    ("bet_binary", "expected"),
    [
        (0x84210, (1, 2, 3, 4, 0)),
        (0x88888, (1, 1, 1, 1, 1)),
        (0x44444, (2, 2, 2, 2, 2)),
        (0x22222, (3, 3, 3, 3, 3)),
        (0x11111, (4, 4, 4, 4, 4)),
        (0x00000, (0, 0, 0, 0, 0)),
    ],
)
def test_binary_to_indices(bet_binary: int, expected: Sequence[int]) -> None:
    assert expected == Math.binary_to_indices(bet_binary)


@pytest.mark.parametrize(
    ("amount_hash", "expected"),
    [
        ("BAQBAQBAQBAQBAQBAQBAQBAQBAQBAQ", (4098,) * 10),
        ("AtmAtmAtmAtmAtmAtmAtmAtmAtmAtm", (1000,) * 10),
        ("AMyAMyAMyAMyAMyAMyAMyAMyAMyAMy", (2000,) * 10),
        ("BfKBfKBfKBfKBfKBfKBfKBfKBfKBfK", (3000,) * 10),
        ("ByWByWByWByWByWByWByWByWByWByW", (4000,) * 10),
        ("BSiBSiBSiBSiBSiBSiBSiBSiBSiBSi", (5000,) * 10),
        ("CluCluCluCluCluCluCluCluCluClu", (6000,) * 10),
        ("CEGCEGCEGCEGCEGCEGCEGCEGCEGCEG", (7000,) * 10),
        ("CXSCXSCXSCXSCXSCXSCXSCXSCXSCXS", (8000,) * 10),
        ("DreDreDreDreDreDreDreDreDreDre", (9000,) * 10),
        ("DKqDKqDKqDKqDKqDKqDKqDKqDKqDKq", (10000,) * 10),
        ("SzCSzCSzCSzCSzCSzCSzCSzCSzCSzC", (50000,) * 10),
        ("ZUiZUiZUiZUiZUiZUiZUiZUiZUiZUi", (70000,) * 10),
    ],
)
def test_amounts_hash_to_bet_amounts(amount_hash: str, expected: Sequence[int]) -> None:
    assert expected == Math.amounts_hash_to_bet_amounts(amount_hash)


@pytest.mark.parametrize(
    ("expected", "bet_amounts"),
    [
        ("BAQBAQBAQBAQBAQBAQBAQBAQBAQBAQ", (4098,) * 10),
        ("AtmAtmAtmAtmAtmAtmAtmAtmAtmAtm", (1000,) * 10),
        ("AMyAMyAMyAMyAMyAMyAMyAMyAMyAMy", (2000,) * 10),
        ("BfKBfKBfKBfKBfKBfKBfKBfKBfKBfK", (3000,) * 10),
        ("ByWByWByWByWByWByWByWByWByWByW", (4000,) * 10),
        ("BSiBSiBSiBSiBSiBSiBSiBSiBSiBSi", (5000,) * 10),
        ("CluCluCluCluCluCluCluCluCluClu", (6000,) * 10),
        ("CEGCEGCEGCEGCEGCEGCEGCEGCEGCEG", (7000,) * 10),
        ("CXSCXSCXSCXSCXSCXSCXSCXSCXSCXS", (8000,) * 10),
        ("DreDreDreDreDreDreDreDreDreDre", (9000,) * 10),
        ("DKqDKqDKqDKqDKqDKqDKqDKqDKqDKq", (10000,) * 10),
        ("SzCSzCSzCSzCSzCSzCSzCSzCSzCSzC", (50000,) * 10),
        ("ZUiZUiZUiZUiZUiZUiZUiZUiZUiZUi", (70000,) * 10),
    ],
)
def test_bet_amounts_to_amounts_hash(expected: str, bet_amounts: Sequence[int]) -> None:
    assert Math.bet_amounts_to_amounts_hash(bet_amounts) == expected


@pytest.mark.parametrize(
    ("bets_hash", "expected"),
    [
        ("", 0),
        ("f", 1),
        ("faa", 1),
        ("ltqvqwgimhqtvrnywrwvijwnn", 10),
        ("ltqvqwgimhqtvrnywrwvijwnnxgslqmrylolnk", 15),
    ],
)
def test_bets_hash_to_bet_count(bets_hash: str, expected: int) -> None:
    assert expected == Math.bets_hash_to_bet_count(bets_hash)


def test_amount_hash_to_bet_amounts_below_50() -> None:
    assert Math.amounts_hash_to_bet_amounts("AaX") == (49,)


def test_build_payout_regions() -> None:
    assert (Math.build_payout_regions([Math.binary_to_indices(0x80000)], [1])) == {
        524287: 0,
        589823: 1,
    }


def test_bet_amount_hash_max_is_the_largest_representable_amount() -> None:
    assert Math.BET_AMOUNT_HASH_MAX == 70303

    hash_ = Math.bet_amounts_to_amounts_hash([Math.BET_AMOUNT_HASH_MAX])
    assert Math.amounts_hash_to_bet_amounts(hash_) == (Math.BET_AMOUNT_HASH_MAX,)


@pytest.mark.parametrize("amount", [70304, 80000, 500_000])
def test_bet_amounts_to_amounts_hash_rejects_unrepresentable_amounts(
    amount: int,
) -> None:
    # these used to wrap around (80000 -> 9696) or collide with "no amount" (70304)
    with pytest.raises(ValueError, match=str(amount)):
        Math.bet_amounts_to_amounts_hash([50, amount])
