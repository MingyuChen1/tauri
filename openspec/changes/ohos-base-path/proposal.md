# OHOS Base Path 工具函数

## 目标
为 tauri-utils 添加 OHOS 平台的 base path 获取函数，供各仓统一使用。

## 背景
当前 OHOS 的 base path 通过 `OnceLock` 在 `tauri-runtime` 中管理，但其他仓（如 tauri-utils 的 path 模块）也需要访问。提供一个统一的工具函数。

## 范围
- 在 `tauri-utils` 的 `path` 模块添加 `ohos_base_path()` 函数
- 函数仅在 `cfg(target_env = "ohos")` 下可用
- 不影响其他平台
