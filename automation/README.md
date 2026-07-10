# Logit coefficient automation

`.github/workflows/update-logit-values.yml` runs at the beginning of each month:
grabs the past month's rounds, retrains the multinomial logit model, and patches
the six `LOGIT_*` arrays directly into
`crates/core/src/models/multinomial_logit.rs`, committing straight to `main`.

Previously lived in [rneopets/neofoodclub](https://github.com/rneopets/neofoodclub)
and only emitted a standalone fragment someone had to copy by hand into this repo;
moved here so the source of truth updates itself in place.

Run it locally with `uv run grab_rounds.py && uv run preprocessing.py && uv run final.py`.
