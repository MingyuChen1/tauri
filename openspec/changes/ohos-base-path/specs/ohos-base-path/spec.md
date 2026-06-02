## ADDED Requirements

### Requirement: OHOS base path getter
The system SHALL provide a function `ohos_base_path()` that returns the OHOS application base path as a `PathBuf`.

#### Scenario: Get base path on OHOS
- **WHEN** called on an OHOS device
- **THEN** returns `/data/storage/el2/base/haps/entry/files` as PathBuf

#### Scenario: Function only available on OHOS
- **WHEN** compiled for non-OHOS targets
- **THEN** the function does not exist (cfg gated)
