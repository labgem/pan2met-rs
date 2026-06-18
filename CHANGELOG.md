# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.0

### Added

- use padmet-rs crate to read [PADMet](https://github.com/AuReMe/padmet.git) files
- configure the inference with a YAML file
- **rules**
    - reject if pathway outside taxonomic range, from NCBI-Taxonomy reference
    - reject if pathway is missing a key reaction
    - reject if pathway either signaling or transport pathway
    - reject if no pathway reaction is catalyzed
    - reject if a synthesis pathway is missing last reaction
    - reject if a catabolysis pathway is missing its first first reaction
    - reject if the `pathway score` is above a threshold
- **pathway score**
    - `pathway score` is the weighted sum of the `reaction score` of the non-spontaneous reactions of the pathway
    - reaction score is the sum of three components: presence score, key reaction score and uniqueness score.
