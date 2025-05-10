# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v5.14.4](https://github.com/PurpleBooth/git-mit/compare/c03760e28e8ec31733ecc853b2d905ac4ffaab55..v5.14.4) - 2025-05-10
#### Bug Fixes
- **(deps)** update rust crate tokio to v1.43.1 [security] - ([e0b020c](https://github.com/PurpleBooth/git-mit/commit/e0b020ca4f187c0f9dccb0ec8ee3bf5323ec9aaf)) - renovate[bot]
- **(deps)** update rust crate openssl to v0.10.72 [security] - ([6c4dfdc](https://github.com/PurpleBooth/git-mit/commit/6c4dfdc0182ca35bc54e43a4608d1bf0f687c649)) - renovate[bot]
- **(deps)** update rust crate clap_complete to v4.5.44 (#1548) - ([70e9773](https://github.com/PurpleBooth/git-mit/commit/70e97733ab33faa74dc73e21a7891f7575251b44)) - renovate[bot]
- **(deps)** update rust crate clap to v4.5.27 (#1546) - ([5d28785](https://github.com/PurpleBooth/git-mit/commit/5d28785366d0f94ed005cf7c535fa702c561e772)) - renovate[bot]
- remove invalid `const` qualifier from `new` function in `in_memory.rs` - ([cb272e0](https://github.com/PurpleBooth/git-mit/commit/cb272e0cce0c06ba914c57d26b395df1a1b13b79)) - Billie Thompson (aider)
- replace std::io::Error with miette! macro for error handling - ([7ff0cd2](https://github.com/PurpleBooth/git-mit/commit/7ff0cd27b98601344aa6fb2c896674a1cc7a3fa4)) - Billie Thompson (aider)
- resolve type conversion errors in set_relates_to expiration handling - ([ffd1171](https://github.com/PurpleBooth/git-mit/commit/ffd1171c526ca5a52bdbe1de822c105d866120c7)) - Billie Thompson (aider)
#### Build system
- specify json and yaml files for prettier - ([a396917](https://github.com/PurpleBooth/git-mit/commit/a3969175cdc80583c4a216ce84c1a7cc33a18c39)) - Billie Thompson (aider)
#### Miscellaneous Chores
- **(deps)** update rust docker tag to v1.86.0 - ([95227c7](https://github.com/PurpleBooth/git-mit/commit/95227c71331511140512d758652bcd337670bc96)) - renovate[bot]
- **(deps)** update actions/cache action to v4.2.0 (#1542) - ([c03760e](https://github.com/PurpleBooth/git-mit/commit/c03760e28e8ec31733ecc853b2d905ac4ffaab55)) - renovate[bot]
- add mutate target to run cargo mutants with shuffle - ([6aa0c8c](https://github.com/PurpleBooth/git-mit/commit/6aa0c8ce03c703064fb4389eb2b229d3cc15671b)) - Billie Thompson (aider)
- update prettier command comment in Justfile - ([0898fed](https://github.com/PurpleBooth/git-mit/commit/0898fede90b149ef401fd15fade66a6d5251bd71)) - Billie Thompson
- remove rustfmt.toml configuration file - ([fe3c939](https://github.com/PurpleBooth/git-mit/commit/fe3c9395a4a732325a1107f0f774d7cc19570fda)) - Billie Thompson
- remove unstable clippy flag from fmt recipes - ([bb4c659](https://github.com/PurpleBooth/git-mit/commit/bb4c6593ee0bcc35eae01e555be4163405c204ee)) - Billie Thompson (aider)
- remove clippy multiple crate versions allowance from justfile - ([f875246](https://github.com/PurpleBooth/git-mit/commit/f8752467d9daf4cd282043e05916a1cf53de35c8)) - Billie Thompson (aider)
- remove explicit clippy lint settings from justfile - ([71348c4](https://github.com/PurpleBooth/git-mit/commit/71348c49c16069282806fca25ac9e7e28e035e31)) - Billie Thompson (aider)
- standardize lint attributes across all crates - ([03ab374](https://github.com/PurpleBooth/git-mit/commit/03ab374705e8aef510fa7b80df267cf4484979e5)) - Billie Thompson (aider)
- add mutants.out* to .gitignore - ([ed05977](https://github.com/PurpleBooth/git-mit/commit/ed0597739be51344405a903d635323207b28aa63)) - Billie Thompson
- add a per file linter - ([5be75c3](https://github.com/PurpleBooth/git-mit/commit/5be75c32e88fceaf8e513ef8f6e8e5d60ef1c9b7)) - Billie Thompson
- add .aider* to .gitignore - ([372ace6](https://github.com/PurpleBooth/git-mit/commit/372ace6f66ea728f1ccd70d604aee544ba37c115)) - Billie Thompson
#### Refactoring
- make `InMemory::new` a const function - ([8e4253b](https://github.com/PurpleBooth/git-mit/commit/8e4253b41b20fa4e0dc66b779cb7807ed80a98ee)) - Billie Thompson
- add ARGS parameter to fmt and lint recipes in Justfile - ([74def91](https://github.com/PurpleBooth/git-mit/commit/74def91434bcfd0c38aa39854afecc734b8f7ea6)) - Billie Thompson (aider)
- update time handling to use OffsetDateTime in set_relates_to.rs - ([3b330b4](https://github.com/PurpleBooth/git-mit/commit/3b330b46769fa5ece7d27284036009f8a216bbde)) - Billie Thompson (aider)
#### Style
- Format code and imports for consistency - ([a25e5ea](https://github.com/PurpleBooth/git-mit/commit/a25e5eab081114f960fb6448ec1c414e45bb382a)) - Billie Thompson

- - -

## [v5.14.3](https://github.com/PurpleBooth/git-mit/compare/47214f861b6a22d5e83c9180ef1792b88b41c596..v5.14.3) - 2025-01-11
#### Bug Fixes
- bump versions and follow clippy advice - ([9d07720](https://github.com/PurpleBooth/git-mit/commit/9d0772009e3d2846529ec6551ec38a7de3adc8c8)) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update rust:1.82.0 docker digest to d9c3c6f (#1532) - ([47214f8](https://github.com/PurpleBooth/git-mit/commit/47214f861b6a22d5e83c9180ef1792b88b41c596)) - renovate[bot]
#### Tests
- update wrappings from miette - ([ef09873](https://github.com/PurpleBooth/git-mit/commit/ef0987369ef949191e17b145ed17e5a3ca558112)) - Billie Thompson
- fix broken formatting - ([eacafb5](https://github.com/PurpleBooth/git-mit/commit/eacafb5b30107ea18ffab8e94389fb7c589996bf)) - Billie Thompson
- match new miette formatting of help - ([43216f9](https://github.com/PurpleBooth/git-mit/commit/43216f91e504ed771a1a1afe0a49d1abc0e8c58c)) - Billie Thompson

- - -

## [v5.14.2](https://github.com/PurpleBooth/git-mit/compare/9aaf0752ca9683d657fbe0eec03d13da41a06f70..v5.14.2) - 2024-11-02
#### Bug Fixes
- **(deps)** update rust crate which to v7 - ([9aaf075](https://github.com/PurpleBooth/git-mit/commit/9aaf0752ca9683d657fbe0eec03d13da41a06f70)) - renovate[bot]

- - -

## [v5.14.1](https://github.com/PurpleBooth/git-mit/compare/f8ae5ae01494fd521c376b4b7c47b245af731f83..v5.14.1) - 2024-10-31
#### Bug Fixes
- correct windows install - ([4b6deda](https://github.com/PurpleBooth/git-mit/commit/4b6deda40acdf2d21d0d89310d4ca81a75198cea)) - Billie Thompson
#### Documentation
- fix the order of the cargo install list - ([de8e6a3](https://github.com/PurpleBooth/git-mit/commit/de8e6a33a907fec1557c6787e63f2a744de0e588)) - Billie Thompson
- Update note about completion - ([b91d212](https://github.com/PurpleBooth/git-mit/commit/b91d2121132d81309edc8ef528eb2e4094cf7853)) - Billie Thompson
- Add link to installer scripts - ([1d2f27c](https://github.com/PurpleBooth/git-mit/commit/1d2f27c437cf39a894cd890ad0261a0bb57db78c)) - Billie Thompson
- Add missing relates to - ([f8ae5ae](https://github.com/PurpleBooth/git-mit/commit/f8ae5ae01494fd521c376b4b7c47b245af731f83)) - Billie Thompson
#### Miscellaneous Chores
- formatting - ([27ca1d5](https://github.com/PurpleBooth/git-mit/commit/27ca1d51f5f00591f1e3a654df8af24a447cc9b6)) - Billie Thompson

- - -

## [v5.14.0](https://github.com/PurpleBooth/git-mit/compare/b4af9ba56565e66832378981f2cf798c538f9864..v5.14.0) - 2024-10-31
#### Bug Fixes
- Correct path - ([b035475](https://github.com/PurpleBooth/git-mit/commit/b035475145aa68bc40218108dcf0147625089d6e)) - Billie Thompson
- Correct the windows installer path - ([57f23e7](https://github.com/PurpleBooth/git-mit/commit/57f23e7a1dc5afaebc2e5a8dd23fccab4d175374)) - Billie Thompson
#### Continuous Integration
- Correct filenames - ([4de6a3e](https://github.com/PurpleBooth/git-mit/commit/4de6a3e0d4b169b13d02f643cfbfd966bdaa860f)) - Billie Thompson
#### Features
- Add an installer for windows - ([538e173](https://github.com/PurpleBooth/git-mit/commit/538e173d8134f60695be1a0addfd4289502b64f9)) - Billie Thompson
- Add a bash/linux installer - ([b4af9ba](https://github.com/PurpleBooth/git-mit/commit/b4af9ba56565e66832378981f2cf798c538f9864)) - Billie Thompson

- - -

## [v5.13.31](https://github.com/PurpleBooth/git-mit/compare/03249782c298c3bb6d9288f79888e97e13bb2363..v5.13.31) - 2024-10-31
#### Bug Fixes
- Bump version to ensure release contains install - ([ee1f7d5](https://github.com/PurpleBooth/git-mit/commit/ee1f7d5f47ab3c50b4a7753d6809d35f99a92e2d)) - Billie Thompson
#### Continuous Integration
- Add missing install binary to release - ([0324978](https://github.com/PurpleBooth/git-mit/commit/03249782c298c3bb6d9288f79888e97e13bb2363)) - Billie Thompson

- - -

## [v5.13.30](https://github.com/PurpleBooth/git-mit/compare/ba99377931bbf484440f457c218e5d62ff8ba639..v5.13.30) - 2024-10-23
#### Bug Fixes
- **(deps)** update rust crate regex to 1.11.0 - ([198a62c](https://github.com/PurpleBooth/git-mit/commit/198a62c8933d586e386970d707225245ecbfab61)) - renovate[bot]
#### Documentation
- Improve the template for bugs - ([ba99377](https://github.com/PurpleBooth/git-mit/commit/ba99377931bbf484440f457c218e5d62ff8ba639)) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update rust docker tag to v1.82.0 - ([f150921](https://github.com/PurpleBooth/git-mit/commit/f150921ef505bbdee93edd5695d480901dd90573)) - renovate[bot]
- **(deps)** update actions/cache action to v4.1.2 - ([e8ae1c8](https://github.com/PurpleBooth/git-mit/commit/e8ae1c8788a8d9cc21362d1519d1108a4ca5e8f6)) - renovate[bot]
- **(deps)** update rust:1.81.0 docker digest to 7b7f7ae - ([8bb529a](https://github.com/PurpleBooth/git-mit/commit/8bb529a5bebc19d8594a21484fe8f721f3c8ab3f)) - renovate[bot]
- **(deps)** update rust docker tag to v1.81.0 - ([21d3f27](https://github.com/PurpleBooth/git-mit/commit/21d3f276b0e4b7315cf299194952be60cf4df378)) - renovate[bot]
- **(src)** Formatting - ([1ef8ab5](https://github.com/PurpleBooth/git-mit/commit/1ef8ab53755b6dd19d6155b1b91dad08bbc925e9)) - Billie Thompson

- - -

## [v5.13.29](https://github.com/PurpleBooth/git-mit/compare/be7457756cc44d213cdb17fda01a57d864f948e0..v5.13.29) - 2024-08-31
#### Bug Fixes
- **(deps)** update rust crate tokio to 1.40.0 - ([be74577](https://github.com/PurpleBooth/git-mit/commit/be7457756cc44d213cdb17fda01a57d864f948e0)) - renovate[bot]

- - -

## [v5.13.28](https://github.com/PurpleBooth/git-mit/compare/12e3849b8b6b3e5b86d319ff5ddff16a1261698c..v5.13.28) - 2024-08-27
#### Bug Fixes
- **(deps)** update serde monorepo to 1.0.209 - ([12e3849](https://github.com/PurpleBooth/git-mit/commit/12e3849b8b6b3e5b86d319ff5ddff16a1261698c)) - renovate[bot]

- - -

## [v5.13.27](https://github.com/PurpleBooth/git-mit/compare/3f4bd844701469ae80d32b56cc8e627e99239d72..v5.13.27) - 2024-08-26
#### Bug Fixes
- **(deps)** update serde monorepo to 1.0.208 - ([9c0d438](https://github.com/PurpleBooth/git-mit/commit/9c0d43815a5b19f0ec2ecc86f43ffeda4ce9c76d)) - renovate[bot]
#### Continuous Integration
- Update to latest version of common pipelines - ([3f4bd84](https://github.com/PurpleBooth/git-mit/commit/3f4bd844701469ae80d32b56cc8e627e99239d72)) - Billie Thompson

- - -

## [v5.13.26](https://github.com/PurpleBooth/git-mit/compare/b231f8bc7e214cffba9bec5d57793eb0198b15c3..v5.13.26) - 2024-08-26
#### Bug Fixes
- **(deps)** update rust crate clap_complete to 4.5.23 - ([b231f8b](https://github.com/PurpleBooth/git-mit/commit/b231f8bc7e214cffba9bec5d57793eb0198b15c3)) - renovate[bot]

- - -

## [v5.13.25](https://github.com/PurpleBooth/git-mit/compare/e8646b8dc62f122af60a72a61d71b826c872c4b0..v5.13.25) - 2024-08-25
#### Bug Fixes
- **(deps)** update rust crate which to 6.0.3 - ([e8646b8](https://github.com/PurpleBooth/git-mit/commit/e8646b8dc62f122af60a72a61d71b826c872c4b0)) - renovate[bot]

- - -

## [v5.13.24](https://github.com/PurpleBooth/git-mit/compare/caf993045777cc4a77a0569b67301da9f4af068b..v5.13.24) - 2024-08-25
#### Bug Fixes
- **(deps)** update rust crate tokio to 1.39.3 - ([caf9930](https://github.com/PurpleBooth/git-mit/commit/caf993045777cc4a77a0569b67301da9f4af068b)) - renovate[bot]

- - -

## [v5.13.23](https://github.com/PurpleBooth/git-mit/compare/377445bede83e1f0438ac8cf17a6dad800e52b35..v5.13.23) - 2024-08-24
#### Bug Fixes
- **(deps)** update rust crate clap_complete to 4.5.20 - ([0f15149](https://github.com/PurpleBooth/git-mit/commit/0f15149d01eca696c51ddb6dbf938852564be6f4)) - renovate[bot]
#### Continuous Integration
- Add sbom step - ([377445b](https://github.com/PurpleBooth/git-mit/commit/377445bede83e1f0438ac8cf17a6dad800e52b35)) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update purplebooth/common-pipelines action to v0.9.4 - ([d3c20a4](https://github.com/PurpleBooth/git-mit/commit/d3c20a49db6675269463024a135709f9b62283e4)) - renovate[bot]
- **(deps)** update rust crate tempfile to 3.12.0 - ([0d84882](https://github.com/PurpleBooth/git-mit/commit/0d8488230aeaf79a925653ccc029288bfb7a30d6)) - renovate[bot]
- **(deps)** update actions/cache action to v4.0.2 - ([b451331](https://github.com/PurpleBooth/git-mit/commit/b451331f4dc41858e48f6269db3cbc0eb423ede1)) - renovate[bot]
- **(deps)** update purplebooth/common-pipelines action to v0.9.2 - ([4a33d40](https://github.com/PurpleBooth/git-mit/commit/4a33d40e8aebbba8692108fa279e31c6c2904cd1)) - renovate[bot]

- - -

## [v5.13.22](https://github.com/PurpleBooth/git-mit/compare/82e9a345e3303c9d79f756b1f69ac56e40835bd5..v5.13.22) - 2024-08-21
#### Bug Fixes
- **(deps)** update rust crate clap_complete to v4.5.20 - ([cb6c645](https://github.com/PurpleBooth/git-mit/commit/cb6c645eddcc3d7e43ceaee7e8f4253707b630dd)) - renovate[bot]
#### Continuous Integration
- Remove old dependabot file - ([5d1ff47](https://github.com/PurpleBooth/git-mit/commit/5d1ff47b80906bec5b24be9471493dc3665cb3f7)) - Billie Thompson
- Allow merge groups - ([b52ea62](https://github.com/PurpleBooth/git-mit/commit/b52ea62e9827e7392d4f32c3a75106e5ee7ae62e)) - Billie Thompson
- Add missing v to changelog generation - ([559394e](https://github.com/PurpleBooth/git-mit/commit/559394e9c22134c2ea2afe2fc2a9ff95d3da68d8)) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update purplebooth/common-pipelines action to v0.9.0 - ([76670ff](https://github.com/PurpleBooth/git-mit/commit/76670ff3070560175c6e23c89b4bd9a1fba1b8b6)) - renovate[bot]
- **(deps)** update purplebooth/generate-formula-action action to v0.1.14 - ([82e9a34](https://github.com/PurpleBooth/git-mit/commit/82e9a345e3303c9d79f756b1f69ac56e40835bd5)) - renovate[bot]

- - -

## [v5.13.21](https://github.com/PurpleBooth/git-mit/compare/384d2d6de73603ea79cb2b820e8cea1efb8c9ec8..v5.13.21) - 2024-08-20
#### Bug Fixes
- **(deps)** update rust crate clap_complete to v4.5.19 - ([774ffe7](https://github.com/PurpleBooth/git-mit/commit/774ffe7e8dcfb8e4e88b43a60841265b70424193)) - renovate[bot]
#### Continuous Integration
- Use cog for changelog - ([384d2d6](https://github.com/PurpleBooth/git-mit/commit/384d2d6de73603ea79cb2b820e8cea1efb8c9ec8)) - Billie Thompson

- - -

## [v5.13.20](https://github.com/PurpleBooth/git-mit/compare/06d1484602212a477589ac51457b50c99ad4aecf..v5.13.20) - 2024-08-18
#### Bug Fixes
- **(deps)** update rust crate which to v6.0.3 - ([06d1484](https://github.com/PurpleBooth/git-mit/commit/06d1484602212a477589ac51457b50c99ad4aecf)) - renovate[bot]

- - -

## [v5.13.19](https://github.com/PurpleBooth/git-mit/compare/d7c0bd13a6c7bb90ed460ad2981b2afb19944730..v5.13.19) - 2024-08-18
#### Bug Fixes
- **(deps)** update rust crate tokio to v1.39.3 - ([d7c0bd1](https://github.com/PurpleBooth/git-mit/commit/d7c0bd13a6c7bb90ed460ad2981b2afb19944730)) - renovate[bot]

- - -

## [v5.13.18](https://github.com/PurpleBooth/git-mit/compare/e17c5abea7b673330bb91e662d01f27c228f474e..v5.13.18) - 2024-08-18
#### Bug Fixes
- **(deps)** update rust crate clap_complete to v4.5.18 - ([e17c5ab](https://github.com/PurpleBooth/git-mit/commit/e17c5abea7b673330bb91e662d01f27c228f474e)) - renovate[bot]

- - -

## [v5.13.17](https://github.com/PurpleBooth/git-mit/compare/4366e67ddac9bb549d1c5794965ed717bdbd0be1..v5.13.17) - 2024-08-17
#### Bug Fixes
- **(deps)** update serde monorepo to v1.0.208 - ([4366e67](https://github.com/PurpleBooth/git-mit/commit/4366e67ddac9bb549d1c5794965ed717bdbd0be1)) - renovate[bot]

- - -

## [v5.13.16](https://github.com/PurpleBooth/git-mit/compare/cd82e195ab3a0f29084c8648a3de66ad2fad9856..v5.13.16) - 2024-08-16
#### Bug Fixes
- **(deps)** bump clap from 4.5.15 to 4.5.16 - ([20ac82e](https://github.com/PurpleBooth/git-mit/commit/20ac82e8b5c70f37c9f6975c8179677119d5ba49)) - dependabot[bot]
#### Continuous Integration
- **(deps)** bump PurpleBooth/common-pipelines from 0.8.27 to 0.8.28 - ([cd82e19](https://github.com/PurpleBooth/git-mit/commit/cd82e195ab3a0f29084c8648a3de66ad2fad9856)) - dependabot[bot]
- Do not use steps in a release context - ([947ba58](https://github.com/PurpleBooth/git-mit/commit/947ba5808744e0d7c4ade11709c10e60e06cba11)) - Billie Thompson
- remove version bump - ([6962f26](https://github.com/PurpleBooth/git-mit/commit/6962f2647d15721abc283e999a7fedb4a00c2589)) - Billie Thompson

- - -

## [v5.13.15](https://github.com/PurpleBooth/git-mit/compare/803a2889b8f709e0838b455f5b33ab63c82ad7b7..v5.13.15) - 2024-08-13
#### Bug Fixes
- **(deps)** update serde monorepo to v1.0.207 - ([803a288](https://github.com/PurpleBooth/git-mit/commit/803a2889b8f709e0838b455f5b33ab63c82ad7b7)) - renovate[bot]

- - -

## [v5.13.14](https://github.com/PurpleBooth/git-mit/compare/235b635489822ec8274c760bd91f972933acb1ac..v5.13.14) - 2024-08-13
#### Bug Fixes
- **(deps)** update rust crate clap_complete to v4.5.16 - ([235b635](https://github.com/PurpleBooth/git-mit/commit/235b635489822ec8274c760bd91f972933acb1ac)) - renovate[bot]

- - -

## [v5.13.13](https://github.com/PurpleBooth/git-mit/compare/5c7bc96f3173da361c06202d39da806166082588..v5.13.13) - 2024-08-12
#### Bug Fixes
- **(deps)** update rust crate clap_complete to v4.5.14 - ([8f3a936](https://github.com/PurpleBooth/git-mit/commit/8f3a9366a57fe40f7f44d80f9c9e409725e07afc)) - renovate[bot]
- **(deps)** update rust crate clap to v4.5.15 - ([5c7bc96](https://github.com/PurpleBooth/git-mit/commit/5c7bc96f3173da361c06202d39da806166082588)) - renovate[bot]

- - -

## [v5.13.12](https://github.com/PurpleBooth/git-mit/compare/34dbf11dfb00600fd249466c4107222b70bb0b4b..v5.13.12) - 2024-08-11
#### Bug Fixes
- **(deps)** update serde monorepo to v1.0.205 - ([943216a](https://github.com/PurpleBooth/git-mit/commit/943216a3b42d5646d100e01a07d7203dee1c067a)) - renovate[bot]
#### Build system
- Bump dependency to openssl3 to prevent linkage problems - ([0eb4dd5](https://github.com/PurpleBooth/git-mit/commit/0eb4dd55b4685923ff7551b66e4d82e16a59527a)) - Billie Thompson
- Correct casing and update environment variable formatting - ([0b5b6e2](https://github.com/PurpleBooth/git-mit/commit/0b5b6e2b369416d0f5cbde442f3e09428a1a5888)) - Billie Thompson
- Homebrew formatting - ([34dbf11](https://github.com/PurpleBooth/git-mit/commit/34dbf11dfb00600fd249466c4107222b70bb0b4b)) - Billie Thompson

- - -

## [v5.13.11](https://github.com/PurpleBooth/git-mit/compare/9a88f9f7e4bea33e2cb45653ec69286e3dac2ac0..v5.13.11) - 2024-08-09
#### Bug Fixes
- **(deps)** bump rust from 1.80.0 to 1.80.1 - ([2bbd67e](https://github.com/PurpleBooth/git-mit/commit/2bbd67e498c2ab112cc93251b3f9159a8537900b)) - dependabot[bot]
- **(deps)** bump clap_complete from 4.5.12 to 4.5.13 - ([9a88f9f](https://github.com/PurpleBooth/git-mit/commit/9a88f9f7e4bea33e2cb45653ec69286e3dac2ac0)) - dependabot[bot]

- - -

## [v5.13.10](https://github.com/PurpleBooth/git-mit/compare/54d34489c59fb46c1bc248d8ffeb9f10f0e1e56d..v5.13.10) - 2024-08-09
#### Bug Fixes
- **(deps)** bump clap from 4.5.13 to 4.5.14 - ([01e3d80](https://github.com/PurpleBooth/git-mit/commit/01e3d804213e65132c683751bcf41790f78d1d8b)) - dependabot[bot]
#### Continuous Integration
- **(deps)** bump PurpleBooth/common-pipelines from 0.8.26 to 0.8.27 - ([57b2307](https://github.com/PurpleBooth/git-mit/commit/57b230746e09263c77a586307e94d966da2891f1)) - dependabot[bot]
- **(deps)** bump PurpleBooth/common-pipelines from 0.8.25 to 0.8.26 - ([4bacc07](https://github.com/PurpleBooth/git-mit/commit/4bacc07c5da8ce2c5f8d55040d47e10a2c08156f)) - dependabot[bot]
#### Miscellaneous Chores
- **(deps)** update rust crate tempfile to v3.12.0 - ([54d3448](https://github.com/PurpleBooth/git-mit/commit/54d34489c59fb46c1bc248d8ffeb9f10f0e1e56d)) - renovate[bot]

- - -

## [v5.13.9](https://github.com/PurpleBooth/git-mit/compare/1a8307406e7120f730d06a3e52a303ad9dd09111..v5.13.9) - 2024-08-05
#### Bug Fixes
- **(deps)** bump regex from 1.10.5 to 1.10.6 - ([be15042](https://github.com/PurpleBooth/git-mit/commit/be15042d915d5073f972ea07222c8665917a211c)) - dependabot[bot]
- **(deps)** bump tempfile from 3.10.1 to 3.11.0 - ([1a83074](https://github.com/PurpleBooth/git-mit/commit/1a8307406e7120f730d06a3e52a303ad9dd09111)) - dependabot[bot]

- - -

## [v5.13.8](https://github.com/PurpleBooth/git-mit/compare/7afee160b0bb5897d206e64d70740b2adf4166ab..v5.13.8) - 2024-08-05
#### Bug Fixes
- **(deps)** update rust crate regex to v1.10.6 - ([a919210](https://github.com/PurpleBooth/git-mit/commit/a91921054b6c281ef8f72bac1c95728f0b00af4c)) - renovate[bot]
#### Continuous Integration
- **(Mergify)** configuration update - ([080cb80](https://github.com/PurpleBooth/git-mit/commit/080cb80b8fe6262342641bda87fe5430840be65e)) - Billie Thompson
- **(deps)** bump PurpleBooth/generate-formula-action from 0.1.11 to 0.1.13 - ([29abb7d](https://github.com/PurpleBooth/git-mit/commit/29abb7d801394f8c35193106a1cb877a2c816b7f)) - dependabot[bot]
- **(deps)** bump PurpleBooth/common-pipelines from 0.8.15 to 0.8.23 - ([96b37ab](https://github.com/PurpleBooth/git-mit/commit/96b37ab5a193c56c2a70e9d898e91ea3077ff6cc)) - dependabot[bot]
- don't use interstitial .version - ([54dbca2](https://github.com/PurpleBooth/git-mit/commit/54dbca220f80f7feed7f6164dc071b1f3fea5dff)) - Billie Thompson
- Merge releases - ([7afee16](https://github.com/PurpleBooth/git-mit/commit/7afee160b0bb5897d206e64d70740b2adf4166ab)) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update purplebooth/common-pipelines action to v0.8.25 - ([36ffbfe](https://github.com/PurpleBooth/git-mit/commit/36ffbfe482d0c47abe14946bf654bbb026535514)) - renovate[bot]

- - -

## [v5.13.7](https://github.com/PurpleBooth/git-mit/compare/4130dc11a0477220fa723ddc51a0a145208941ac..v5.13.7) - 2024-08-02
#### Bug Fixes
- Publish attestations - ([921cae3](https://github.com/PurpleBooth/git-mit/commit/921cae3a8a41bdd053b53186c2b71449757eb7b8)) - Billie Thompson
#### Continuous Integration
- Switch to faster actions - ([ffe7c88](https://github.com/PurpleBooth/git-mit/commit/ffe7c88732c68135855068b78b65f8645bd739bb)) - Billie Thompson
- Remove old style set-output - ([0a60675](https://github.com/PurpleBooth/git-mit/commit/0a606753545adf0e4bb138f2c3931615694e996b)) - Billie Thompson
- Add permissions for signing - ([9c807bb](https://github.com/PurpleBooth/git-mit/commit/9c807bb3b9211d6a3eaa33a461f6c7899ff16082)) - Billie Thompson
- Create renovate.json - ([4130dc1](https://github.com/PurpleBooth/git-mit/commit/4130dc11a0477220fa723ddc51a0a145208941ac)) - Billie Thompson

- - -

## [v5.13.6](https://github.com/PurpleBooth/git-mit/compare/43e88224933cf0fa5baecbb59428cb654c92921e..v5.13.6) - 2024-08-01
#### Bug Fixes
- **(deps)** bump clap from 4.5.11 to 4.5.13 - ([43e8822](https://github.com/PurpleBooth/git-mit/commit/43e88224933cf0fa5baecbb59428cb654c92921e)) - dependabot[bot]

- - -

## [v5.13.5](https://github.com/PurpleBooth/git-mit/compare/0fe4cf57589b3451efbd4b0a47c0415412854fd3..v5.13.5) - 2024-08-01
#### Bug Fixes
- **(deps)** bump toml from 0.8.17 to 0.8.19 - ([83f0253](https://github.com/PurpleBooth/git-mit/commit/83f02531d3f137ac4846edd373c2f62f163d6bf4)) - dependabot[bot]
- **(deps)** bump clap_complete from 4.5.11 to 4.5.12 - ([35a6dac](https://github.com/PurpleBooth/git-mit/commit/35a6dacdcb85d7ab1aad5e5818dfcea1fd48f30a)) - dependabot[bot]
#### Continuous Integration
- **(deps)** bump PurpleBooth/common-pipelines from 0.8.9 to 0.8.15 - ([8db87c1](https://github.com/PurpleBooth/git-mit/commit/8db87c1d9ff9740656885d06c90ae76a82ca01ac)) - dependabot[bot]
- **(deps)** bump PurpleBooth/common-pipelines from 0.6.53 to 0.8.9 - ([0fe4cf5](https://github.com/PurpleBooth/git-mit/commit/0fe4cf57589b3451efbd4b0a47c0415412854fd3)) - dependabot[bot]

- - -

## [v5.13.4](https://github.com/PurpleBooth/git-mit/compare/a8443ffcb170f36f13de7e015b763c1175c13f7d..v5.13.4) - 2024-07-31
#### Bug Fixes
- **(deps)** bump toml from 0.8.16 to 0.8.17 - ([a8443ff](https://github.com/PurpleBooth/git-mit/commit/a8443ffcb170f36f13de7e015b763c1175c13f7d)) - dependabot[bot]

- - -

## [v5.13.3](https://github.com/PurpleBooth/git-mit/compare/86074737699b6170455724eb463ca1e97b6addc1..v5.13.3) - 2024-07-30
#### Bug Fixes
- **(deps)** bump which from 6.0.1 to 6.0.2 - ([8607473](https://github.com/PurpleBooth/git-mit/commit/86074737699b6170455724eb463ca1e97b6addc1)) - dependabot[bot]

- - -

## [v5.13.2](https://github.com/PurpleBooth/git-mit/compare/db557535795a63719a4fc65a810efea8bcb820a8..v5.13.2) - 2024-07-29
#### Bug Fixes
- **(deps)** bump tokio from 1.39.1 to 1.39.2 - ([db55753](https://github.com/PurpleBooth/git-mit/commit/db557535795a63719a4fc65a810efea8bcb820a8)) - dependabot[bot]

- - -

## [v5.13.1](https://github.com/PurpleBooth/git-mit/compare/11939547b2d4e7224ffb90a277e0b6a7a651dff9..v5.13.1) - 2024-07-26
#### Bug Fixes
- Update deps - ([1193954](https://github.com/PurpleBooth/git-mit/commit/11939547b2d4e7224ffb90a277e0b6a7a651dff9)) - Billie Thompson

- - -

## [v5.13.0](https://github.com/PurpleBooth/git-mit/compare/9cb9326dd200beda33ee1938034df2bf5ba58964..v5.13.0) - 2024-07-26
#### Features
- Allow users to select what the behaviour should be on rebase - ([9cb9326](https://github.com/PurpleBooth/git-mit/commit/9cb9326dd200beda33ee1938034df2bf5ba58964)) - Billie Thompson

- - -

## [v5.12.220](https://github.com/PurpleBooth/git-mit/compare/9cbaa0278a6db8fb44005a3098f5fce16eb9140e..v5.12.220) - 2024-07-26
#### Bug Fixes
- **(deps)** bump clap from 4.5.10 to 4.5.11 - ([d7fdaf6](https://github.com/PurpleBooth/git-mit/commit/d7fdaf686b58a0a6f10f04feb2cb158033ee4b9b)) - dependabot[bot]
- **(deps)** bump toml from 0.8.15 to 0.8.16 - ([e3acbc0](https://github.com/PurpleBooth/git-mit/commit/e3acbc0e419987e8a3e3c54ab9d22e6f0397e62a)) - dependabot[bot]
- **(deps)** bump rust from 1.79.0 to 1.80.0 - ([1491801](https://github.com/PurpleBooth/git-mit/commit/1491801c9cf562c19d05e123a36bef8f42291197)) - dependabot[bot]
- **(deps)** bump clap_complete from 4.5.9 to 4.5.11 - ([9cbaa02](https://github.com/PurpleBooth/git-mit/commit/9cbaa0278a6db8fb44005a3098f5fce16eb9140e)) - dependabot[bot]

- - -

## [v5.12.219](https://github.com/PurpleBooth/git-mit/compare/540645e72b52ca5e55da0014cffc336133067ab8..v5.12.219) - 2024-07-24
#### Bug Fixes
- **(deps)** bump clap_complete from 4.5.8 to 4.5.9 - ([540645e](https://github.com/PurpleBooth/git-mit/commit/540645e72b52ca5e55da0014cffc336133067ab8)) - dependabot[bot]

- - -

## [v5.12.218](https://github.com/PurpleBooth/git-mit/compare/06b6eb3133e6be0addc0db8b8e8ff159882fdae5..v5.12.218) - 2024-07-24
#### Bug Fixes
- **(deps)** bump tokio from 1.38.1 to 1.39.1 - ([9cff646](https://github.com/PurpleBooth/git-mit/commit/9cff64618f81fff457a96ffc71c49626f864aa7c)) - dependabot[bot]
- **(deps)** bump clap from 4.5.9 to 4.5.10 - ([06b6eb3](https://github.com/PurpleBooth/git-mit/commit/06b6eb3133e6be0addc0db8b8e8ff159882fdae5)) - dependabot[bot]

- - -

## [v5.12.217](https://github.com/PurpleBooth/git-mit/compare/66a3d4f3df1f21ba5f962057e9f9c46b4fa86624..v5.12.217) - 2024-07-22
#### Bug Fixes
- **(deps)** bump openssl from 0.10.64 to 0.10.66 - ([66a3d4f](https://github.com/PurpleBooth/git-mit/commit/66a3d4f3df1f21ba5f962057e9f9c46b4fa86624)) - dependabot[bot]

- - -

## [v5.12.216](https://github.com/PurpleBooth/git-mit/compare/1f4aa1ed78d383aaefec590e7109ce7926fd149c..v5.12.216) - 2024-07-18
#### Bug Fixes
- **(deps)** bump toml from 0.8.14 to 0.8.15 - ([16450f8](https://github.com/PurpleBooth/git-mit/commit/16450f86ecef3bfaba2ca5f064e67adf7ed77595)) - dependabot[bot]
- **(deps)** bump thiserror from 1.0.62 to 1.0.63 - ([1f4aa1e](https://github.com/PurpleBooth/git-mit/commit/1f4aa1ed78d383aaefec590e7109ce7926fd149c)) - dependabot[bot]

- - -

## [v5.12.215](https://github.com/PurpleBooth/git-mit/compare/464e706d26452263ef2a125b7a0095168fc28cfc..v5.12.215) - 2024-07-17
#### Bug Fixes
- **(deps)** bump tokio from 1.38.0 to 1.38.1 - ([464e706](https://github.com/PurpleBooth/git-mit/commit/464e706d26452263ef2a125b7a0095168fc28cfc)) - dependabot[bot]

- - -

## [v5.12.214](https://github.com/PurpleBooth/git-mit/compare/0d044eba681398c50d40b800b53a603634259635..v5.12.214) - 2024-07-12
#### Bug Fixes
- **(deps)** bump thiserror from 1.0.61 to 1.0.62 - ([0d044eb](https://github.com/PurpleBooth/git-mit/commit/0d044eba681398c50d40b800b53a603634259635)) - dependabot[bot]

- - -

## [v5.12.213](https://github.com/PurpleBooth/git-mit/compare/d44dc7ad16b73aaaf811ce8eb614589a235a1c13..v5.12.213) - 2024-07-11
#### Bug Fixes
- **(deps)** bump clap_complete from 4.5.7 to 4.5.8 - ([d44dc7a](https://github.com/PurpleBooth/git-mit/commit/d44dc7ad16b73aaaf811ce8eb614589a235a1c13)) - dependabot[bot]

- - -

## [v5.12.212](https://github.com/PurpleBooth/git-mit/compare/5341bf6b4414519d1be1d6d390b2873405fe9cac..v5.12.212) - 2024-07-09
#### Bug Fixes
- **(deps)** bump clap from 4.5.8 to 4.5.9 - ([5341bf6](https://github.com/PurpleBooth/git-mit/commit/5341bf6b4414519d1be1d6d390b2873405fe9cac)) - dependabot[bot]

- - -

## [v5.12.211](https://github.com/PurpleBooth/git-mit/compare/4de416edc3a88f4734f8e3fd47f9d2a329b9b997..v5.12.211) - 2024-07-01
#### Bug Fixes
- **(deps)** bump clap from 4.5.7 to 4.5.8 - ([8d3abe8](https://github.com/PurpleBooth/git-mit/commit/8d3abe8d70dfe3377d4f359be4fdceb51c8e7118)) - dependabot[bot]
- **(deps)** bump clap_complete from 4.5.6 to 4.5.7 - ([4de416e](https://github.com/PurpleBooth/git-mit/commit/4de416edc3a88f4734f8e3fd47f9d2a329b9b997)) - dependabot[bot]

- - -

## [v5.12.210](https://github.com/PurpleBooth/git-mit/compare/c2cb0c9a0b768462e7b12e9c58b16d4ad3cbdb8e..v5.12.210) - 2024-06-20
#### Bug Fixes
- **(deps)** bump clap_complete from 4.5.5 to 4.5.6 - ([c2cb0c9](https://github.com/PurpleBooth/git-mit/commit/c2cb0c9a0b768462e7b12e9c58b16d4ad3cbdb8e)) - dependabot[bot]

- - -

## [v5.12.209](https://github.com/PurpleBooth/git-mit/compare/96a62933966e3dc06faf4da98ca129d160b573c8..v5.12.209) - 2024-06-14
#### Bug Fixes
- **(deps)** bump rust from 1.78.0 to 1.79.0 - ([96a6293](https://github.com/PurpleBooth/git-mit/commit/96a62933966e3dc06faf4da98ca129d160b573c8)) - dependabot[bot]

- - -

## [v5.12.208](https://github.com/PurpleBooth/git-mit/compare/9c358f8765af3d47565a255c6b05d38592164524..v5.12.208) - 2024-06-14
#### Bug Fixes
- **(deps)** bump git2 from 0.18.3 to 0.19.0 - ([9c358f8](https://github.com/PurpleBooth/git-mit/commit/9c358f8765af3d47565a255c6b05d38592164524)) - dependabot[bot]

- - -

## [v5.12.207](https://github.com/PurpleBooth/git-mit/compare/b699622d9b5d7b3701d902a4d9e9fd0583250782..v5.12.207) - 2024-06-11
#### Bug Fixes
- **(deps)** bump clap from 4.5.6 to 4.5.7 - ([b699622](https://github.com/PurpleBooth/git-mit/commit/b699622d9b5d7b3701d902a4d9e9fd0583250782)) - dependabot[bot]

- - -

## [v5.12.206](https://github.com/PurpleBooth/git-mit/compare/33c93adfa5468a6fa48d5028efbae1283053d41f..v5.12.206) - 2024-06-10
#### Bug Fixes
- **(deps)** bump regex from 1.10.4 to 1.10.5 - ([3bd8a9b](https://github.com/PurpleBooth/git-mit/commit/3bd8a9bd9a1649fc6123a66aa7eca44671039946)) - dependabot[bot]
- **(deps)** bump clap_complete from 4.5.4 to 4.5.5 - ([33c93ad](https://github.com/PurpleBooth/git-mit/commit/33c93adfa5468a6fa48d5028efbae1283053d41f)) - dependabot[bot]

- - -

## [v5.12.205](https://github.com/PurpleBooth/git-mit/compare/5cc968be214e7777111f0c91ab74bdbaed101031..v5.12.205) - 2024-06-07
#### Bug Fixes
- **(deps)** bump clap_complete from 4.5.2 to 4.5.4 - ([5cc968b](https://github.com/PurpleBooth/git-mit/commit/5cc968be214e7777111f0c91ab74bdbaed101031)) - dependabot[bot]

- - -

## [v5.12.204](https://github.com/PurpleBooth/git-mit/compare/a642240c5de5f2686f9207cff54d056bb276f730..v5.12.204) - 2024-06-04
#### Bug Fixes
- **(deps)** bump toml from 0.8.13 to 0.8.14 - ([a642240](https://github.com/PurpleBooth/git-mit/commit/a642240c5de5f2686f9207cff54d056bb276f730)) - dependabot[bot]

- - -

## [v5.12.203](https://github.com/PurpleBooth/git-mit/compare/ff5601ae06078afa075528ab89d5e9096e56d452..v5.12.203) - 2024-05-31
#### Bug Fixes
- **(deps)** bump tokio from 1.37.0 to 1.38.0 - ([ff5601a](https://github.com/PurpleBooth/git-mit/commit/ff5601ae06078afa075528ab89d5e9096e56d452)) - dependabot[bot]

- - -

## [v5.12.202](https://github.com/PurpleBooth/git-mit/compare/39902d3ab843568d94be79ec3a84d9d85ef93ba2..v5.12.202) - 2024-05-20
#### Bug Fixes
- **(deps)** bump thiserror from 1.0.60 to 1.0.61 - ([39902d3](https://github.com/PurpleBooth/git-mit/commit/39902d3ab843568d94be79ec3a84d9d85ef93ba2)) - dependabot[bot]

- - -

## [v5.12.201](https://github.com/PurpleBooth/git-mit/compare/aab36efbb23db0109acf00d9e6746f8ea94aa0b4..v5.12.201) - 2024-05-16
#### Bug Fixes
- **(deps)** bump toml from 0.8.12 to 0.8.13 - ([aab36ef](https://github.com/PurpleBooth/git-mit/commit/aab36efbb23db0109acf00d9e6746f8ea94aa0b4)) - dependabot[bot]

- - -

## [v5.12.200](https://github.com/PurpleBooth/git-mit/compare/1e22317af0f1db26e90ba4553c39bab8039cea68..v5.12.200) - 2024-05-07
#### Bug Fixes
- **(deps)** bump thiserror from 1.0.59 to 1.0.60 - ([1e22317](https://github.com/PurpleBooth/git-mit/commit/1e22317af0f1db26e90ba4553c39bab8039cea68)) - dependabot[bot]

- - -

## [v5.12.199](https://github.com/PurpleBooth/git-mit/compare/40896b0c79f6b8d9701f255c87ed205224ffc508..v5.12.199) - 2024-05-03
#### Bug Fixes
- **(deps)** bump rust from 1.77.2 to 1.78.0 - ([40896b0](https://github.com/PurpleBooth/git-mit/commit/40896b0c79f6b8d9701f255c87ed205224ffc508)) - dependabot[bot]

- - -

## [v5.12.198](https://github.com/PurpleBooth/git-mit/compare/522ed6268397786ccc0824962d8bd685cf207539..v5.12.198) - 2024-04-29
#### Bug Fixes
- **(deps)** bump arboard from 3.3.2 to 3.4.0 - ([522ed62](https://github.com/PurpleBooth/git-mit/commit/522ed6268397786ccc0824962d8bd685cf207539)) - dependabot[bot]

- - -

## [v5.12.197](https://github.com/PurpleBooth/git-mit/compare/73632c5b30058663db97e5529517a2bc0d340bb9..v5.12.197) - 2024-04-22
#### Bug Fixes
- **(deps)** bump thiserror from 1.0.58 to 1.0.59 - ([73632c5](https://github.com/PurpleBooth/git-mit/commit/73632c5b30058663db97e5529517a2bc0d340bb9)) - dependabot[bot]

- - -

## [v5.12.196](https://github.com/PurpleBooth/git-mit/compare/c68ded83e351dde32e4446e0808eda10d2332571..v5.12.196) - 2024-04-13
#### Bug Fixes
- Update rust to new name in homebrew - ([c68ded8](https://github.com/PurpleBooth/git-mit/commit/c68ded83e351dde32e4446e0808eda10d2332571)) - Billie Thompson

- - -

## [v5.12.195](https://github.com/PurpleBooth/git-mit/compare/5e54a6adabb94bff38f2226c5a9a6af16ed2117c..v5.12.195) - 2024-04-12
#### Bug Fixes
- **(deps)** bump rust from 1.77.0 to 1.77.2 - ([5e54a6a](https://github.com/PurpleBooth/git-mit/commit/5e54a6adabb94bff38f2226c5a9a6af16ed2117c)) - dependabot[bot]

- - -

## [v5.12.194](https://github.com/PurpleBooth/git-mit/compare/1a9492e9975f34bef534e714fb9921e64388d0f2..v5.12.194) - 2024-04-11
#### Bug Fixes
- **(deps)** bump time from 0.3.35 to 0.3.36 - ([b45b93f](https://github.com/PurpleBooth/git-mit/commit/b45b93fd9f874243dc86eb9c18c60e645f7935e5)) - dependabot[bot]
#### Continuous Integration
- Make artifact names unique - ([1a9492e](https://github.com/PurpleBooth/git-mit/commit/1a9492e9975f34bef534e714fb9921e64388d0f2)) - Billie Thompson

- - -

## [v5.12.193](https://github.com/PurpleBooth/git-mit/compare/bec768dda0b1bedf8643f23e13f7ff7934bf5826..v5.12.193) - 2024-04-11
#### Bug Fixes
- **(deps)** bump rust from 1.76.0 to 1.77.1 - ([bec768d](https://github.com/PurpleBooth/git-mit/commit/bec768dda0b1bedf8643f23e13f7ff7934bf5826)) - dependabot[bot]

- - -

## [v5.12.192](https://github.com/PurpleBooth/git-mit/compare/ab9956981471c5c732b9fcfef1ba6dc6598688fc..v5.12.192) - 2024-04-10
#### Bug Fixes
- Correct the location of backtics in the mit-prepare-commit-msg binary - ([8856ab1](https://github.com/PurpleBooth/git-mit/commit/8856ab1a67f81c6f59aa053840305a0bcbaaed14)) - Billie Thompson
- Bump versions and fix lints - ([f1c7926](https://github.com/PurpleBooth/git-mit/commit/f1c792655ea3b895bd1a2ef0c8cf7fb57cf56e9c)) - Billie Thompson
#### Continuous Integration
- **(deps)** bump PurpleBooth/generate-formula-action from 0.1.10 to 0.1.11 - ([f4ff0c4](https://github.com/PurpleBooth/git-mit/commit/f4ff0c4f5c43644da31b045f3a51e493f119722c)) - dependabot[bot]
- **(deps)** bump actions/upload-artifact from 3 to 4 - ([ab99569](https://github.com/PurpleBooth/git-mit/commit/ab9956981471c5c732b9fcfef1ba6dc6598688fc)) - dependabot[bot]
- Use main for rust checks - ([fbe03c8](https://github.com/PurpleBooth/git-mit/commit/fbe03c81eb70d8a1763584e2a21a28a54c5a0acc)) - Billie Thompson
- Ensure we are formatting on nightly - ([cf68285](https://github.com/PurpleBooth/git-mit/commit/cf682853ac7c3dba1dfec61205b337e15e021626)) - Billie Thompson
#### Refactoring
- Reformat code - ([2cda71d](https://github.com/PurpleBooth/git-mit/commit/2cda71df603397023347d6cd96902479d0a78041)) - Billie Thompson

- - -

## [v5.12.191](https://github.com/PurpleBooth/git-mit/compare/v5.12.190..v5.12.191) - 2024-02-15
#### Bug Fixes
- **(deps)** bump clap_complete from 4.4.6 to 4.5.0 - ([5a389c8](https://github.com/PurpleBooth/git-mit/commit/5a389c855fb6b0036f3d9033e72df039ac5dbba7)) - dependabot[bot]
- **(deps)** bump which from 5.0.0 to 6.0.0 - ([f4e0c36](https://github.com/PurpleBooth/git-mit/commit/f4e0c36e375c14a8d7cef4fe7a34f0e7b830a4e0)) - dependabot[bot]
- **(deps)** bump arboard from 3.3.0 to 3.3.1 - ([bc87b0c](https://github.com/PurpleBooth/git-mit/commit/bc87b0c6834f35479cd3ab02843d8da10da5491f)) - dependabot[bot]
- **(deps)** bump thiserror from 1.0.56 to 1.0.57 - ([01007c9](https://github.com/PurpleBooth/git-mit/commit/01007c9a0bd8e6e417978fe773faf44bc4dd00e3)) - dependabot[bot]

- - -

## [v5.12.190](https://github.com/PurpleBooth/git-mit/compare/v5.12.189..v5.12.190) - 2024-02-15
#### Bug Fixes
- **(deps)** bump clap from 4.4.14 to 4.5.0 - ([f0de0ee](https://github.com/PurpleBooth/git-mit/commit/f0de0eeb690cbb990c589c7084de55c69bfc3f61)) - dependabot[bot]
- **(deps)** bump tempfile from 3.9.0 to 3.10.0 - ([26f06c2](https://github.com/PurpleBooth/git-mit/commit/26f06c2af84908ce1c4c4026714f0c238affd8f8)) - dependabot[bot]
- **(deps)** bump serde_yaml from 0.9.30 to 0.9.31 - ([f820ef0](https://github.com/PurpleBooth/git-mit/commit/f820ef067ce60527b6afdb23be29ac40caa37562)) - dependabot[bot]

- - -

## [v5.12.189](https://github.com/PurpleBooth/git-mit/compare/v5.12.188..v5.12.189) - 2024-02-15
#### Bug Fixes
- **(deps)** bump git2 from 0.18.1 to 0.18.2 - ([3a72d38](https://github.com/PurpleBooth/git-mit/commit/3a72d38e2bf87fed6fa43b5d1b615b0ba7e63940)) - dependabot[bot]

- - -

## [v5.12.188](https://github.com/PurpleBooth/git-mit/compare/v5.12.187..v5.12.188) - 2024-02-15
#### Bug Fixes
- **(deps)** bump rust from 1.74.0 to 1.76.0 - ([0ce12e9](https://github.com/PurpleBooth/git-mit/commit/0ce12e9d9ce4229373b6bba46d95eac494e7820f)) - dependabot[bot]
- **(deps)** bump toml from 0.8.8 to 0.8.10 - ([b7d75a0](https://github.com/PurpleBooth/git-mit/commit/b7d75a03698d6bb22f5ac1abd2d6eade29c280b1)) - dependabot[bot]
- **(deps)** bump tokio from 1.35.1 to 1.36.0 - ([b8a55bd](https://github.com/PurpleBooth/git-mit/commit/b8a55bd8e1d9567bf1aceffaca75696a18b5d353)) - dependabot[bot]
- **(deps)** bump time from 0.3.31 to 0.3.34 - ([5fc778d](https://github.com/PurpleBooth/git-mit/commit/5fc778dbdea21bc1cc50c08d105bdd9cfcb533de)) - dependabot[bot]
- **(deps)** bump regex from 1.10.2 to 1.10.3 - ([e867935](https://github.com/PurpleBooth/git-mit/commit/e8679357d0c75a718237fadf3792880fc39f9134)) - dependabot[bot]
#### Continuous Integration
- **(deps)** bump ncipollo/release-action from 1.13.0 to 1.14.0 - ([92d0755](https://github.com/PurpleBooth/git-mit/commit/92d0755f6c415f06a7a3513c0cb7c56640c4d4ae)) - dependabot[bot]
- **(deps)** bump nick-invision/retry from 2.9.0 to 3.0.0 - ([b18a10b](https://github.com/PurpleBooth/git-mit/commit/b18a10bb033fbe27c48019261ee7b4e4114aa37a)) - dependabot[bot]
- **(deps)** bump actions/cache from 3 to 4 - ([5f16a8c](https://github.com/PurpleBooth/git-mit/commit/5f16a8c6569169300bfe5b8f790a67e623339823)) - dependabot[bot]

- - -

## [v5.12.187](https://github.com/PurpleBooth/git-mit/compare/v5.12.186..v5.12.187) - 2024-02-15
#### Bug Fixes
- **(deps)** bump libgit2-sys from 0.16.1+1.7.1 to 0.16.2+1.7.2 - ([01ca0e6](https://github.com/PurpleBooth/git-mit/commit/01ca0e6c7d3c272e3ec38d76bd8f6f343eb55672)) - dependabot[bot]
#### Continuous Integration
- **(Mergify)** configuration update (#1369) - ([107470a](https://github.com/PurpleBooth/git-mit/commit/107470a84de92be2e3c376ff09c43579a9732dcb)) - Billie Thompson
- Remove markdown linting step - ([81135b1](https://github.com/PurpleBooth/git-mit/commit/81135b102e7afd21c5260c38349124c12bc3b20e)) - Billie Thompson

- - -

## [v5.12.186](https://github.com/PurpleBooth/git-mit/compare/v5.12.185..v5.12.186) - 2024-01-23
#### Bug Fixes
- **(Homebrew)** Correct url to match what homebrew expects - ([27f43c7](https://github.com/PurpleBooth/git-mit/commit/27f43c7198727fa381971cbe2acebac39eb87c0e)) - Billie Thompson
- **(deps)** bump openssl from 0.10.62 to 0.10.63 - ([76f8736](https://github.com/PurpleBooth/git-mit/commit/76f8736d3391ca0cfc3413a17d780c174bbb159e)) - dependabot[bot]

- - -

## [v5.12.185](https://github.com/PurpleBooth/git-mit/compare/v5.12.184..v5.12.185) - 2024-01-22
#### Bug Fixes
- **(deps)** bump thiserror from 1.0.52 to 1.0.56 - ([fb32184](https://github.com/PurpleBooth/git-mit/commit/fb32184f1a4210b4856307f3bbfd835c0a39cb0e)) - dependabot[bot]
- **(deps)** bump clap from 4.4.13 to 4.4.14 - ([e40d31c](https://github.com/PurpleBooth/git-mit/commit/e40d31ceaf9f5b4f9f2f5b80e7e7758cc2117c75)) - dependabot[bot]
- **(deps)** bump clap_complete from 4.4.5 to 4.4.6 - ([b29463b](https://github.com/PurpleBooth/git-mit/commit/b29463be757d4342cb0f9767936ce950dfba316b)) - dependabot[bot]
- **(deps)** bump clap from 4.4.12 to 4.4.13 - ([0211a9b](https://github.com/PurpleBooth/git-mit/commit/0211a9b1c01b019a85ca8ec27ed41629759d37a1)) - dependabot[bot]
- **(src)** Clippy - ([d9fca9d](https://github.com/PurpleBooth/git-mit/commit/d9fca9d34d53651d02db85211ff347a255719570)) - Billie Thompson

- - -

## [v5.12.184](https://github.com/PurpleBooth/git-mit/compare/v5.12.183..v5.12.184) - 2024-01-02
#### Bug Fixes
- **(deps)** bump clap from 4.4.11 to 4.4.12 - ([fab63f4](https://github.com/PurpleBooth/git-mit/commit/fab63f4b674841ef29b507e13faba4252ecea2bf)) - dependabot[bot]

- - -

## [v5.12.183](https://github.com/PurpleBooth/git-mit/compare/v5.12.182..v5.12.183) - 2024-01-02
#### Bug Fixes
- **(deps)** bump serde_yaml from 0.9.29 to 0.9.30 - ([23a1ffa](https://github.com/PurpleBooth/git-mit/commit/23a1fface3f5ab1a638ea345c71aa7b658876b16)) - dependabot[bot]

- - -

## [v5.12.182](https://github.com/PurpleBooth/git-mit/compare/v5.12.181..v5.12.182) - 2023-12-28
#### Bug Fixes
- **(deps)** bump clap_complete from 4.4.4 to 4.4.5 - ([b57badb](https://github.com/PurpleBooth/git-mit/commit/b57badba7b7494c08ab8f7f48904c080eda4c9cb)) - dependabot[bot]
- **(deps)** bump tempfile from 3.8.1 to 3.9.0 - ([b842c46](https://github.com/PurpleBooth/git-mit/commit/b842c466c34dbd3e8a04472d485abca0bf6894fa)) - dependabot[bot]
- **(deps)** bump serde_yaml from 0.9.27 to 0.9.29 - ([8642f1d](https://github.com/PurpleBooth/git-mit/commit/8642f1dd0ac019a8df30562ce6640016a9f9d57b)) - dependabot[bot]
- **(deps)** bump openssl from 0.10.61 to 0.10.62 - ([735399b](https://github.com/PurpleBooth/git-mit/commit/735399b49d59836acf02e5177442b9cd28a0b15f)) - dependabot[bot]
- **(deps)** bump thiserror from 1.0.51 to 1.0.52 - ([d8a27b7](https://github.com/PurpleBooth/git-mit/commit/d8a27b733e2abe11a0c402dec52aafa6935c8e3b)) - dependabot[bot]
- **(deps)** bump unsafe-libyaml from 0.2.9 to 0.2.10 - ([a8031da](https://github.com/PurpleBooth/git-mit/commit/a8031da909ca541e427648d73ace26c46f436523)) - dependabot[bot]
- **(deps)** bump tokio from 1.35.0 to 1.35.1 - ([5ad202e](https://github.com/PurpleBooth/git-mit/commit/5ad202e9761bf9365ba37b606794d0fbb203e532)) - dependabot[bot]

- - -

## [v5.12.181](https://github.com/PurpleBooth/git-mit/compare/v5.12.180..v5.12.181) - 2023-12-19
#### Bug Fixes
- **(deps)** bump time from 0.3.30 to 0.3.31 - ([702bf0f](https://github.com/PurpleBooth/git-mit/commit/702bf0f1cbf2f239a25c8c9ca606a7deae4a0ad2)) - dependabot[bot]
- **(deps)** bump thiserror from 1.0.50 to 1.0.51 - ([d3e44ad](https://github.com/PurpleBooth/git-mit/commit/d3e44ad623c1505284d9300972f765d8ce3c0afc)) - dependabot[bot]
#### Continuous Integration
- **(deps)** bump actions/download-artifact from 3 to 4 - ([3346e7a](https://github.com/PurpleBooth/git-mit/commit/3346e7a63949f3cdf31b182e1ab5b7d9c67fb390)) - dependabot[bot]

- - -

## [v5.12.180](https://github.com/PurpleBooth/git-mit/compare/v5.12.179..v5.12.180) - 2023-12-11
#### Bug Fixes
- **(deps)** bump tokio from 1.34.0 to 1.35.0 - ([a42c217](https://github.com/PurpleBooth/git-mit/commit/a42c2173d55d0e766c20663a781e6089c6bd0886)) - dependabot[bot]

- - -

## [v5.12.179](https://github.com/PurpleBooth/git-mit/compare/v5.12.178..v5.12.179) - 2023-12-05
#### Bug Fixes
- **(deps)** bump clap from 4.4.10 to 4.4.11 - ([000147b](https://github.com/PurpleBooth/git-mit/commit/000147b2b018347feb28cc9e108ecb02f9243a20)) - dependabot[bot]
- **(deps)** bump openssl from 0.10.60 to 0.10.61 - ([b9aba2c](https://github.com/PurpleBooth/git-mit/commit/b9aba2c0a8967f06486ccc4de6e187597ca17435)) - dependabot[bot]

- - -

## [v5.12.178](https://github.com/PurpleBooth/git-mit/compare/v5.12.177..v5.12.178) - 2023-11-29
#### Bug Fixes
- **(deps)** bump clap from 4.4.8 to 4.4.10 - ([e1f1323](https://github.com/PurpleBooth/git-mit/commit/e1f1323406154770ab506c454337903ea829caf3)) - dependabot[bot]

- - -

## [v5.12.177](https://github.com/PurpleBooth/git-mit/compare/v5.12.176..v5.12.177) - 2023-11-23
#### Bug Fixes
- **(deps)** bump openssl from 0.10.59 to 0.10.60 - ([221df88](https://github.com/PurpleBooth/git-mit/commit/221df88e9a2c2ea9d97221f2e2b322af668f4396)) - dependabot[bot]

- - -

## [v5.12.176](https://github.com/PurpleBooth/git-mit/compare/v5.12.175..v5.12.176) - 2023-11-21
#### Bug Fixes
- **(deps)** bump arboard from 3.2.1 to 3.3.0 - ([6e8bb6d](https://github.com/PurpleBooth/git-mit/commit/6e8bb6dd9c79d56afec3970445899125a6338840)) - dependabot[bot]

- - -

## [v5.12.175](https://github.com/PurpleBooth/git-mit/compare/v5.12.174..v5.12.175) - 2023-11-17
#### Bug Fixes
- **(deps)** bump rust from 1.73.0 to 1.74.0 - ([d9339dc](https://github.com/PurpleBooth/git-mit/commit/d9339dced920b4361afe6e47dedb5f77ea63dbe5)) - dependabot[bot]

- - -

## [v5.12.174](https://github.com/PurpleBooth/git-mit/compare/v5.12.173..v5.12.174) - 2023-11-13
#### Bug Fixes
- **(deps)** bump clap from 4.4.7 to 4.4.8 - ([d1b17f4](https://github.com/PurpleBooth/git-mit/commit/d1b17f4aa5941560b4072ee28c8e279e0b2275c4)) - dependabot[bot]
#### Documentation
- Make image smaller - ([e81b8a4](https://github.com/PurpleBooth/git-mit/commit/e81b8a4d7fed189aeb26b636e00f1a71991fde66)) - Billie Thompson

- - -

## [v5.12.173](https://github.com/PurpleBooth/git-mit/compare/v5.12.172..v5.12.173) - 2023-11-10
#### Bug Fixes
- **(deps)** bump tokio from 1.33.0 to 1.34.0 - ([c64e3af](https://github.com/PurpleBooth/git-mit/commit/c64e3af600dec6add715b4d86b130716111e1dee)) - dependabot[bot]

- - -

## [v5.12.172](https://github.com/PurpleBooth/git-mit/compare/v5.12.171..v5.12.172) - 2023-11-07
#### Bug Fixes
- **(deps)** bump toml from 0.8.6 to 0.8.8 - ([e5219ae](https://github.com/PurpleBooth/git-mit/commit/e5219ae8aa0081c90476259311b82b2b2b1b6ed6)) - dependabot[bot]

- - -

## [v5.12.171](https://github.com/PurpleBooth/git-mit/compare/v5.12.170..v5.12.171) - 2023-11-06
#### Bug Fixes
- **(deps)** bump openssl from 0.10.58 to 0.10.59 - ([2532f86](https://github.com/PurpleBooth/git-mit/commit/2532f867e99254be06cf7d4293cd40bbcc9df4eb)) - dependabot[bot]

- - -

## [v5.12.170](https://github.com/PurpleBooth/git-mit/compare/v5.12.169..v5.12.170) - 2023-11-02
#### Bug Fixes
- **(deps)** bump openssl from 0.10.57 to 0.10.58 - ([ed065cb](https://github.com/PurpleBooth/git-mit/commit/ed065cbaa8230292e4d50b6ba9fe0761b93480a0)) - dependabot[bot]

- - -

## [v5.12.169](https://github.com/PurpleBooth/git-mit/compare/v5.12.168..v5.12.169) - 2023-10-30
#### Bug Fixes
- **(deps)** bump toml from 0.8.5 to 0.8.6 - ([cf6370e](https://github.com/PurpleBooth/git-mit/commit/cf6370ed2828f0add8b7c383d1055dfaab877e3d)) - dependabot[bot]

- - -

## [v5.12.168](https://github.com/PurpleBooth/git-mit/compare/v5.12.167..v5.12.168) - 2023-10-27
#### Bug Fixes
- **(deps)** bump tempfile from 3.8.0 to 3.8.1 - ([b7bf706](https://github.com/PurpleBooth/git-mit/commit/b7bf70617cb74480c6a86d86fa2b34c6a238fe37)) - dependabot[bot]
- **(deps)** bump toml from 0.8.4 to 0.8.5 - ([b0155cc](https://github.com/PurpleBooth/git-mit/commit/b0155cc9f8dff5a0477e6191f86049de015b9f04)) - dependabot[bot]

- - -

## [v5.12.167](https://github.com/PurpleBooth/git-mit/compare/v5.12.166..v5.12.167) - 2023-10-26
#### Bug Fixes
- **(deps)** bump serde_yaml from 0.9.25 to 0.9.27 - ([bbb6db3](https://github.com/PurpleBooth/git-mit/commit/bbb6db34c79bf9305efd7559f58af773c2895c43)) - dependabot[bot]

- - -

## [v5.12.166](https://github.com/PurpleBooth/git-mit/compare/v5.12.165..v5.12.166) - 2023-10-25
#### Bug Fixes
- **(deps)** bump clap from 4.4.6 to 4.4.7 - ([a20fe9f](https://github.com/PurpleBooth/git-mit/commit/a20fe9fc3a70d25aa42e80d0ced21bd9b81ed739)) - dependabot[bot]
- **(deps)** bump clap_complete from 4.4.3 to 4.4.4 - ([e4fdff7](https://github.com/PurpleBooth/git-mit/commit/e4fdff7a97fd4aa22682ae371fb40ae0f7775343)) - dependabot[bot]

- - -

## [v5.12.165](https://github.com/PurpleBooth/git-mit/compare/v5.12.164..v5.12.165) - 2023-10-24
#### Bug Fixes
- **(deps)** bump toml from 0.8.2 to 0.8.4 - ([d7dbdff](https://github.com/PurpleBooth/git-mit/commit/d7dbdff690d70275b3fd2458d32aca961fd0a92d)) - dependabot[bot]

- - -

## [v5.12.164](https://github.com/PurpleBooth/git-mit/compare/v5.12.163..v5.12.164) - 2023-10-23
#### Bug Fixes
- **(deps)** bump comfy-table from 7.0.1 to 7.1.0 - ([074631b](https://github.com/PurpleBooth/git-mit/commit/074631b19c898887597e09da6a2321f2e8c09067)) - dependabot[bot]

- - -

## [v5.12.163](https://github.com/PurpleBooth/git-mit/compare/v5.12.162..v5.12.163) - 2023-10-20
#### Bug Fixes
- **(deps)** bump thiserror from 1.0.49 to 1.0.50 - ([a5470e2](https://github.com/PurpleBooth/git-mit/commit/a5470e2c4bc274adff90c36c4fa2b9e6dd3840ec)) - dependabot[bot]

- - -

## [v5.12.162](https://github.com/PurpleBooth/git-mit/compare/v5.12.161..v5.12.162) - 2023-10-19
#### Bug Fixes
- Error if  runs outside a git repo (#1298) - ([fe2d1bc](https://github.com/PurpleBooth/git-mit/commit/fe2d1bc1769620b7605028001108779b5b5a5e41)) - Sam Bryant

- - -

## [v5.12.161](https://github.com/PurpleBooth/git-mit/compare/v5.12.160..v5.12.161) - 2023-10-18
#### Bug Fixes
- **(deps)** bump which from 4.4.2 to 5.0.0 - ([0cd9f95](https://github.com/PurpleBooth/git-mit/commit/0cd9f9573a5945958e62611809570f30a242518c)) - dependabot[bot]

- - -

## [v5.12.160](https://github.com/PurpleBooth/git-mit/compare/v5.12.159..v5.12.160) - 2023-10-17
#### Bug Fixes
- **(deps)** bump regex from 1.10.1 to 1.10.2 - ([da0356a](https://github.com/PurpleBooth/git-mit/commit/da0356a6417704a4b15603f7fd6b19e83f090020)) - dependabot[bot]

- - -

## [v5.12.159](https://github.com/PurpleBooth/git-mit/compare/v5.12.158..v5.12.159) - 2023-10-16
#### Bug Fixes
- **(deps)** bump time from 0.3.29 to 0.3.30 - ([8d39e00](https://github.com/PurpleBooth/git-mit/commit/8d39e007af75ac083ebb702f1bcf7a9ee7473236)) - dependabot[bot]
- **(deps)** bump regex from 1.10.0 to 1.10.1 - ([271d59c](https://github.com/PurpleBooth/git-mit/commit/271d59c6945fa5e5ccdbb02e2c932b3a75f8585e)) - dependabot[bot]

- - -

## [v5.12.158](https://github.com/PurpleBooth/git-mit/compare/v5.12.157..v5.12.158) - 2023-10-10
#### Bug Fixes
- **(deps)** bump regex from 1.9.6 to 1.10.0 - ([7631008](https://github.com/PurpleBooth/git-mit/commit/76310081f11e1bd94a016201399c6cdeb1d4d0b0)) - dependabot[bot]

- - -

## [v5.12.157](https://github.com/PurpleBooth/git-mit/compare/v5.12.156..v5.12.157) - 2023-10-09
#### Bug Fixes
- **(deps)** bump tokio from 1.32.0 to 1.33.0 - ([e7c79cf](https://github.com/PurpleBooth/git-mit/commit/e7c79cf52be77e84ddce3dd97b6f0556536b5453)) - dependabot[bot]

- - -

## [v5.12.156](https://github.com/PurpleBooth/git-mit/compare/v5.12.155..v5.12.156) - 2023-10-06
#### Bug Fixes
- **(deps)** bump rust from 1.72.1 to 1.73.0 - ([ffa2ad1](https://github.com/PurpleBooth/git-mit/commit/ffa2ad174936c572ae91c1a1b7bd5df7fcd34e7f)) - dependabot[bot]

- - -

## [v5.12.155](https://github.com/PurpleBooth/git-mit/compare/v5.12.154..v5.12.155) - 2023-10-03
#### Bug Fixes
- **(deps)** bump toml from 0.8.1 to 0.8.2 - ([4794bb3](https://github.com/PurpleBooth/git-mit/commit/4794bb3eeed7528c0f421cb0bc7a58ab6194753d)) - dependabot[bot]

- - -

## [v5.12.154](https://github.com/PurpleBooth/git-mit/compare/v5.12.153..v5.12.154) - 2023-10-02
#### Bug Fixes
- **(deps)** bump regex from 1.9.5 to 1.9.6 - ([6385458](https://github.com/PurpleBooth/git-mit/commit/6385458bcc9622ad8f14ca96e229942a3a639faf)) - dependabot[bot]

- - -

## [v5.12.153](https://github.com/PurpleBooth/git-mit/compare/v5.12.152..v5.12.153) - 2023-09-29
#### Bug Fixes
- **(deps)** bump clap_complete from 4.4.2 to 4.4.3 - ([b7d5852](https://github.com/PurpleBooth/git-mit/commit/b7d585239d3e1c09c535b2f51e1f511e6e4cc41c)) - dependabot[bot]
- **(deps)** bump clap from 4.4.5 to 4.4.6 - ([37ed198](https://github.com/PurpleBooth/git-mit/commit/37ed1986fd2a0f6509643c5b9802569a7f966dc3)) - dependabot[bot]

- - -

## [v5.12.152](https://github.com/PurpleBooth/git-mit/compare/v5.12.151..v5.12.152) - 2023-09-27
#### Bug Fixes
- **(deps)** bump mit-lint from 3.2.3 to 3.2.7 - ([11968d2](https://github.com/PurpleBooth/git-mit/commit/11968d29dd73a0fb69bb0ce6479a3405982062e0)) - dependabot[bot]
- **(deps)** bump indoc from 2.0.3 to 2.0.4 - ([4dabed8](https://github.com/PurpleBooth/git-mit/commit/4dabed88726db568428b127303c405483cd153d5)) - dependabot[bot]

- - -

## [v5.12.151](https://github.com/PurpleBooth/git-mit/compare/v5.12.150..v5.12.151) - 2023-09-27
#### Bug Fixes
- **(deps)** bump toml from 0.8.0 to 0.8.1 - ([b770986](https://github.com/PurpleBooth/git-mit/commit/b77098662d1690bd876d67f2af68ac5ef170621a)) - dependabot[bot]
- **(deps)** bump which from 4.4.0 to 4.4.2 - ([50d6142](https://github.com/PurpleBooth/git-mit/commit/50d6142738a6421cf2cb2d922ac1aa558c44f5b1)) - dependabot[bot]
- **(deps)** bump time from 0.3.28 to 0.3.29 - ([0383de0](https://github.com/PurpleBooth/git-mit/commit/0383de023ee2f67297f2d8c75409d35df171dece)) - dependabot[bot]

- - -

## [v5.12.150](https://github.com/PurpleBooth/git-mit/compare/v5.12.149..v5.12.150) - 2023-09-27
#### Bug Fixes
- **(deps)** bump mit-commit from 3.1.7 to 3.1.8 - ([7f4de85](https://github.com/PurpleBooth/git-mit/commit/7f4de855bf2b014d7c94545c2e25b7e3abaadfed)) - dependabot[bot]
- **(deps)** bump thiserror from 1.0.48 to 1.0.49 - ([bb37e30](https://github.com/PurpleBooth/git-mit/commit/bb37e30a1773410cb0b94c57bd95087ebca3af03)) - dependabot[bot]
#### Continuous Integration
- **(deps)** bump nick-invision/retry from 2.8.2 to 2.9.0 - ([035a4e9](https://github.com/PurpleBooth/git-mit/commit/035a4e9a637f25c552826dc828ae9c72a13a2774)) - dependabot[bot]

- - -

## [v5.12.149](https://github.com/PurpleBooth/git-mit/compare/v5.12.148..v5.12.149) - 2023-09-26
#### Bug Fixes
- **(deps)** bump comfy-table from 6.2.0 to 7.0.1 - ([4e42234](https://github.com/PurpleBooth/git-mit/commit/4e42234fe07ff93633655c60e67a09595bf9c518)) - dependabot[bot]
- **(deps)** bump clap from 4.4.1 to 4.4.5 - ([f65af73](https://github.com/PurpleBooth/git-mit/commit/f65af732e5a5f758484f22e1c16d6c81dfb7cfd8)) - dependabot[bot]
- **(deps)** bump clap_complete from 4.4.1 to 4.4.2 - ([c2eee98](https://github.com/PurpleBooth/git-mit/commit/c2eee98eab990a02598d055255e752fc76868d7f)) - dependabot[bot]
- **(deps)** bump criterion from 0.4.0 to 0.5.1 - ([8e7bdce](https://github.com/PurpleBooth/git-mit/commit/8e7bdce62fcd3fff1e4617d7455c5341f793a9be)) - dependabot[bot]
- **(deps)** bump toml from 0.7.6 to 0.8.0 - ([66c94ff](https://github.com/PurpleBooth/git-mit/commit/66c94ff4564873ecbf8d7aa4ae853595521d4db9)) - dependabot[bot]

- - -

## [v5.12.148](https://github.com/PurpleBooth/git-mit/compare/v5.12.147..v5.12.148) - 2023-09-26
#### Bug Fixes
- **(deps)** bump git2 from 0.16.1 to 0.18.1 - ([4152d17](https://github.com/PurpleBooth/git-mit/commit/4152d17165e76aab28edfe0756316783a03332ca)) - dependabot[bot]
- **(deps)** bump rust from 1.72.0 to 1.72.1 - ([c120932](https://github.com/PurpleBooth/git-mit/commit/c120932aefd9232c810bf67c0e9f1f737baedd07)) - dependabot[bot]
- **(deps)** bump clap_complete from 4.4.0 to 4.4.1 - ([9a336fb](https://github.com/PurpleBooth/git-mit/commit/9a336fb25dcd1f492dc8fcded535bb7471c9e624)) - dependabot[bot]
- **(deps)** bump thiserror from 1.0.47 to 1.0.48 - ([2fc9255](https://github.com/PurpleBooth/git-mit/commit/2fc92552980edceeae8f908daf44be1f2b1d1781)) - dependabot[bot]
- **(deps)** bump regex from 1.9.4 to 1.9.5 - ([b8c8574](https://github.com/PurpleBooth/git-mit/commit/b8c85749e088f330ada648d87b21d9732defd780)) - dependabot[bot]
#### Continuous Integration
- **(deps)** bump actions/checkout from 3 to 4 - ([507f10a](https://github.com/PurpleBooth/git-mit/commit/507f10ae8e0da496bfbcd6f963e797102f2ab2bb)) - dependabot[bot]
- **(deps)** bump ncipollo/release-action from 1.12.0 to 1.13.0 - ([de89a9f](https://github.com/PurpleBooth/git-mit/commit/de89a9f395e6f4a8844ed2dce192fc9b867ce7fe)) - dependabot[bot]
- Remove deleted action check - ([8010a25](https://github.com/PurpleBooth/git-mit/commit/8010a257d2a549132aa762c06bcd17be28e61e35)) - Billie Thompson

- - -

## [v5.12.147](https://github.com/PurpleBooth/git-mit/compare/v5.12.146..v5.12.147) - 2023-08-31
#### Bug Fixes
- Remove broken version check - ([fa20a63](https://github.com/PurpleBooth/git-mit/commit/fa20a63802dce1e578791775ccbc1a82df178776)) - Billie Thompson
- Follow clippy advice - ([52fde7e](https://github.com/PurpleBooth/git-mit/commit/52fde7e5288ac3fb1299e5f4bd3a0f63ed399087)) - Billie Thompson
- Remove unneeded mutability - ([a213fca](https://github.com/PurpleBooth/git-mit/commit/a213fca04b318f8f6d14282a8024e55652ab8c50)) - Billie Thompson
- Clippy advice - ([8d695c3](https://github.com/PurpleBooth/git-mit/commit/8d695c396f6607eb45f02facd7a71c42a51e459e)) - Billie Thompson
#### Build system
- Update rust version in docker - ([322ad4d](https://github.com/PurpleBooth/git-mit/commit/322ad4daa57221088896a1a89589e2c4bfee134a)) - Billie Thompson
#### Continuous Integration
- Try switching to Cog - ([06b32a2](https://github.com/PurpleBooth/git-mit/commit/06b32a23060a36d6683ccb3cce2a4f788a9b5568)) - Billie Thompson
- Switch to new conventional tool - ([be0d17f](https://github.com/PurpleBooth/git-mit/commit/be0d17f3a53aef046a064a598b3c85b6a1008a9c)) - Billie Thompson
- Remove deprecated key in mergify config - ([4df46c8](https://github.com/PurpleBooth/git-mit/commit/4df46c8c8756fdc501328f7e21d2c24c9fa64faa)) - Billie Thompson
#### Tests
- Ensure we check the text of a panic - ([1f5bf86](https://github.com/PurpleBooth/git-mit/commit/1f5bf863e89470e6b84eb393c0fb99c2d390ee1a)) - Billie Thompson

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).