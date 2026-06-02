/// Returns the OHOS application base path.
#[cfg(target_env = "ohos")]
pub fn ohos_base_path() -> std::path::PathBuf {
    std::path::PathBuf::from("/data/storage/el2/base/haps/entry/files")
}
