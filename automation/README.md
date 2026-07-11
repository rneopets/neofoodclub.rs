# Logit coefficient automation

`.github/workflows/update-logit-values.yml` runs at the beginning of each month:
fetches every completed round in one shot from
[cdn.neofood.club/previous.jsonl](https://cdn.neofood.club/previous.jsonl) (maintained
by the [neofoodclub-previous-jsonl](https://github.com/rneopets/neofoodclub-previous-jsonl)
Lambda), retrains the multinomial logit model, and patches the six `LOGIT_*` arrays
directly into `crates/core/src/models/multinomial_logit.rs`, committing straight to
`main`.

Previously lived in [rneopets/neofoodclub](https://github.com/rneopets/neofoodclub)
and only emitted a standalone fragment someone had to copy by hand into this repo;
moved here so the source of truth updates itself in place. Also previously fetched
and locally cached one `rounds/{round}.json` file per round (accumulating thousands
of committed files over time) - previous.jsonl replaces that with a single aggregate
file, fetched fresh on every run instead of persisted in git.

Run it locally with `uv run fetch_previous.py && uv run preprocessing.py && uv run final.py`.
