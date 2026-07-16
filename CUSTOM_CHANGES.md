# 自定义改动记录

合并上游 tag 时必须保留以下所有改动。合并后用此命令验证：

```bash
git diff <new-tag> HEAD -- \
  rust/net/src/env.rs \
  rust/attest/src/constants.rs \
  rust/net/infra/src/dns/dns_utils.rs \
  rust/net/res/signal.cer \
  .gitignore \
  LibSignalClient.podspec \
  java/build.gradle \
  java/Makefile \
  node/package.json \
  node/package-lock.json \
  node/Dockerfile \
  .github/workflows/ios_artifacts.yml \
  .github/workflows/jni_artifacts.yml \
  .github/workflows/npm.yml \
  .github/workflows/build_and_test.yml \
  .github/workflows/android_integration.yml \
  .github/workflows/slow_tests.yml
```

> **Runner 规格原则（所有 workflow 通用）**：付费 larger runner 一律替换为免费规格，上游升级时不跟随：
> - `ubuntu-latest-4-cores` / `ubuntu-latest-8-cores` / `ubuntu-24.04-arm64-4-cores` → `ubuntu-latest`
> - `macos-15-xlarge` 等 xlarge 规格 → `macos-14`

> **注意**：上游重构有时会重命名常量（如 `ENCLAVE_ID_SVR2_STAGING` → `ENCLAVE_ID_SVR2_2025Q3_STAGING`）。
> 冲突时不能直接接受上游（`git checkout --theirs`），必须把自定义值迁移到新常量名上。

---

## 1. `rust/net/src/env.rs`

### 规则：所有 prod/staging 域名替换，所有 prod IP 清空，所有 cert 改为 Native

**域名**（所有 `*.signal.org` 替换，上游升级时此规则不变）：

| 常量 | 自定义值 |
|------|---------|
| `DOMAIN_CONFIG_CHAT.connect.hostname` | `chat.ba-chat.com` |
| `DOMAIN_CONFIG_CHAT_STAGING.connect.hostname` | `chat.imba-test.com` |
| `DOMAIN_CONFIG_CDSI.connect.hostname` | `cdsi.ba-chat.com` |
| `DOMAIN_CONFIG_CDSI_STAGING.connect.hostname` | `cdsi.imba-test.com` |
| `DOMAIN_CONFIG_SVR2.connect.hostname` | `svr2.ba-chat.com` |
| `DOMAIN_CONFIG_SVR2_STAGING.connect.hostname` | `svr2.imba-test.com` |
| `DOMAIN_CONFIG_SVRB_PROD.connect.hostname` | `svrb.ba-chat.com` |
| `DOMAIN_CONFIG_SVRB_STAGING.connect.hostname` | `svrb.imba-test.com` |

**IP 地址**（prod 的 3 个清空，staging 保持上游原值不动）：

| 常量 | ip_v4 | ip_v6 |
|------|-------|-------|
| `DOMAIN_CONFIG_CHAT` | `&[]` | `&[]` |
| `DOMAIN_CONFIG_CDSI` | `&[]` | `&[]` |
| `DOMAIN_CONFIG_SVR2` | `&[]` | `&[]` |

**证书**（除 `SVRB_PROD` 外全部改为 Native）：

| 常量 | 自定义值 |
|------|---------|
| `DOMAIN_CONFIG_CHAT.connect.cert` | `RootCertificates::Native` |
| `DOMAIN_CONFIG_CHAT_STAGING.connect.cert` | `RootCertificates::Native` |
| `DOMAIN_CONFIG_CDSI.connect.cert` | `RootCertificates::Native` |
| `DOMAIN_CONFIG_CDSI_STAGING.connect.cert` | `RootCertificates::Native` |
| `DOMAIN_CONFIG_SVR2.connect.cert` | `RootCertificates::Native` |
| `DOMAIN_CONFIG_SVR2_STAGING.connect.cert` | `RootCertificates::Native` |
| `DOMAIN_CONFIG_SVRB_STAGING.connect.cert` | `RootCertificates::Native` |
| `DOMAIN_CONFIG_SVRB_PROD.connect.cert` | 保持 `SIGNAL_ROOT_CERTIFICATES`（该常量 include_bytes! 指向已替换的 `signal.cer`） |

---

## 2. `rust/attest/src/constants.rs`

> **警告**：上游会随时重命名这些常量（加季度后缀等）。
> 合并冲突时必须找到对应新常量名，将下方自定义值写入，不能直接接受上游。

### 规则：SVR2 的 staging/prod enclave 使用自定义值，raft 配置使用单节点参数

**当前常量名**（截至 v0.86.14，下次合并如有重命名需同步更新此处）：

| 常量 | 自定义 enclave ID |
|------|-----------------|
| `ENCLAVE_ID_SVR2_2025Q3_STAGING` | `b49a2d7aa6a92623713541be3342cc2432cbb4052a9ab83b50aef3375651e68f` |
| `ENCLAVE_ID_SVR2_2025Q3_PROD` | `b49a2d7aa6a92623713541be3342cc2432cbb4052a9ab83b50aef3375651e68f` |
| `ENCLAVE_ID_CDSI_PROD` | `3ded708ca5a42fd84b4639dc661a7ec4b9c9f1b92809c0fc91da2349a5a89d05` |

**Raft 配置**（SVR2 staging/prod 用单节点参数，group_id 相同）：

| 常量 | min_voting_replicas | max_voting_replicas | super_majority | group_id |
|------|---------------------|---------------------|----------------|----------|
| `RAFT_CONFIG_SVR2_2025Q3_STAGING` | `1` | `9` | `2` | `9161153614836317716` |
| `RAFT_CONFIG_SVR2_2025Q3_PROD` | `1` | `9` | `2` | `9161153614836317716` |

其余常量（SVRB、2026Q1 等）保持上游值不动。

---

## 3. `rust/net/infra/src/dns/dns_utils.rs`

```rust
const SIGNAL_DOMAIN_SUFFIX: &str = ".imba-test.com";  // 上游是 ".signal.org"
```

---

## 4. `rust/net/res/signal.cer`

二进制文件，替换为自定义 TLS 根证书。被 `SIGNAL_ROOT_CERTIFICATES` 通过 `include_bytes!` 嵌入，影响 `DOMAIN_CONFIG_SVRB_PROD` 的证书验证。合并时此文件无冲突，但要确认没有被覆盖。

---

## 5. `.gitignore`

新增两行（追加在文件末尾）：
```
*.so
/java/android/src/main/testJniLib/*
```

---

## 6. `LibSignalClient.podspec`

| 字段 | 自定义值 |
|------|---------|
| `s.version` | `'<upstream-version>-BA'`（每次合并后在上游版本号后加 `-BA`） |
| `s.homepage` | `https://github.com/g-signal/libsignal` |
| `s.source[:git]` | `https://github.com/g-signal/libsignal.git` |
| fetch script 语法 | `script: %(` （上游是 `script: %q(`，仅第一个 script 改，extract 阶段的第二个保持 `%q(`） |
| fetch URL | `https://github.com/g-signal/libsignal/releases/download/v#{s.version}/${LIBSIGNAL_FFI_PREBUILD_ARCHIVE}` |

---

## 7. `java/build.gradle`

- `group`: 保持 `"io.github.wanggenlin"`（上游是 `"org.signal"`）
- `setUpSigningKey` 函数：保留 PGP 签名诊断日志 + 短格式/长格式 Key ID 兼容逻辑（try/catch 两次尝试）

---

## 8. `java/Makefile`

- `.PHONY` 包含 `test_signing`
- `GRADLE_OPTIONS` 包含 `--stacktrace`
- `publish_java` target 前有环境变量诊断输出
- 存在 `test_signing` target

---

## 9. `node/package.json` 和 `node/package-lock.json`

- `"name"`: 保持 `"@gg-signal/libsignal-client"`（上游是 `"@signalapp/libsignal-client"`，两个文件同步）

---

## 10. `node/Dockerfile`

`dump_syms` 安装方式替换（避免 nightly feature gate 不兼容）：

```dockerfile
# 上游：
RUN cargo install dump_syms --no-default-features --features cli

# 自定义：
RUN rustup install stable && \
    cargo +stable install dump_syms \
        --no-default-features \
        --features cli \
        --locked
```

---

## 11. `.github/workflows/ios_artifacts.yml`

以下是我们独有的改动，上游升级时需保留：

- 触发器：保留 `push: tags: 'v*'`
- `runs-on`: 保持 `macos-14`，**不随上游升级**（`macos-15-xlarge` 等 xlarge 规格需要付费账户，会导致 job 无法启动）
- `dryRun` 判断：保持 `'${{ inputs.dry_run }}' === 'true'`（上游是直接展开布尔值）
- `brew install` 包含 `cmake go perl`
- 步骤 `Set build environment`：设置 `IPHONEOS_DEPLOYMENT_TARGET`、`DEVELOPER_DIR`、`BORING_BSSL_SOURCE_REPLACE`
- 步骤 `Clean build cache`：`cargo clean`
- 移除 Google Cloud Storage 上传（无 `google-github-actions/auth` + `upload-cloud-storage`）
- `Upload to GitHub Release` 同时上传 artifact 本体和 `.sha256`

---

## 12. `.github/workflows/jni_artifacts.yml`

以下是我们独有的改动，上游升级时需保留：

- 触发器：保留 `push: tags: 'v*'`
- `dryRun` 判断：保持 `${{ github.event_name == 'workflow_dispatch' && inputs.dry_run }}`
- `make publish_java` 触发条件：保持 `github.ref_type == 'tag'`（上游是 `!inputs.dry_run`）
- `publish_java` 使用 sonatype 发布（无 GCP auth），env 包含 `SONATYPE_USER`、`SONATYPE_PASSWORD`、`SIGNING_KEYID`、`SIGNING_PASSWORD`、`SIGNING_KEY`
- dry_run 相关步骤条件：`github.event_name == 'workflow_dispatch' && inputs.dry_run`

---

## 13. `.github/workflows/npm.yml`

以下是我们独有的改动，上游升级时需保留：

- 触发器：保留 `push: tags: 'v*'`
- `dryRun`：保持 `${{ inputs.dry_run || false }}`
- `cargo install dump_syms` 保留 `--locked`
- `npm publish --tag`：保持 `'${{ inputs.npm_tag || 'latest' }}'`
