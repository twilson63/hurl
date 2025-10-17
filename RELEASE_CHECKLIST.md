# Release Checklist

## Pre-Release (1-2 days before)

- [ ] Update version in `Cargo.toml` (all workspaces)
  - [ ] Root `Cargo.toml`
  - [ ] `crates/hurl-cli/Cargo.toml`
  - [ ] `crates/hurl-lib/Cargo.toml`

- [ ] Update `Cargo.lock`
  ```bash
  cargo update
  ```

- [ ] Run full test suite
  ```bash
  cargo test --all
  ```

- [ ] Run linter and formatter
  ```bash
  make lint
  make fmt
  ```

- [ ] Build binaries for all platforms
  ```bash
  make build-all
  ```

- [ ] Verify completions are up to date
  ```bash
  cargo run -- --generate-completion bash > completions/hurl.bash
  cargo run -- --generate-completion zsh > completions/hurl.zsh
  cargo run -- --generate-completion fish > completions/hurl.fish
  ```

- [ ] Verify man page is up to date
  ```bash
  cargo run -- --generate-man > man/hurl.1
  ```

- [ ] Update `CHANGELOG.md`
  - [ ] Add version header with date
  - [ ] List all notable changes
  - [ ] List bug fixes
  - [ ] List breaking changes (if any)

- [ ] Create release notes in `RELEASE_NOTES.md`
  - [ ] Write summary of major features
  - [ ] List contributors
  - [ ] Add installation instructions
  - [ ] Note any breaking changes

- [ ] Update `README.md` if needed
  - [ ] Update feature list
  - [ ] Update installation instructions
  - [ ] Update examples if changed

## Release Day

- [ ] Create git tag and push
  ```bash
  git tag -a v0.1.0 -m "Release v0.1.0"
  git push origin v0.1.0
  ```

- [ ] Monitor GitHub Actions workflow
  - [ ] Verify all builds complete successfully
  - [ ] Check artifacts are uploaded correctly
  - [ ] Verify binaries work on each platform

- [ ] Create GitHub release from tag
  - [ ] Use release workflow-generated release notes
  - [ ] Verify all artifacts are attached
  - [ ] Set as latest release if stable

## Post-Release (1-2 days after)

- [ ] Update Homebrew formula
  ```bash
  cd /usr/local/Homebrew/Library/Taps/homebrew/homebrew-core
  # Update Formula/hurl.rb with new version and sha256
  brew audit --strict hurl
  git push
  ```

- [ ] Update Chocolatey package
  - [ ] Update `hurl.nuspec` version
  - [ ] Update `tools/chocolateyInstall.ps1` URL and checksum
  - [ ] Push to Chocolatey
  ```bash
  choco push hurl.0.1.0.nupkg --api-key $env:CHOCOLATEY_API_KEY
  ```

- [ ] Update Debian/Ubuntu packages
  - [ ] Tag package in PPA
  - [ ] Wait for build to complete
  - [ ] Test installation

- [ ] Publish Docker image
  ```bash
  docker build -t hurl:0.1.0 .
  docker tag hurl:0.1.0 hurl:latest
  docker push hurl:0.1.0
  docker push hurl:latest
  ```

- [ ] Announce release
  - [ ] Post on GitHub discussions
  - [ ] Announce on social media/forums
  - [ ] Update project website

- [ ] Update installation docs
  - [ ] Verify Homebrew install works
  - [ ] Verify apt install works
  - [ ] Verify Chocolatey install works
  - [ ] Verify Docker install works

## Verification Commands

### Verify binary functionality
```bash
hurl --version
hurl --help
hurl get https://httpbin.org/get
hurl post https://httpbin.org/post -d '{"test": "data"}'
```

### Verify Homebrew installation
```bash
brew install hurl
hurl --version
brew test hurl
```

### Verify apt installation
```bash
sudo apt-get install hurl
hurl --version
```

### Verify Chocolatey installation
```powershell
choco install hurl
hurl --version
```

### Verify Docker
```bash
docker run --rm hurl:latest --version
docker run --rm hurl:latest get https://httpbin.org/get
```

## Rollback Procedure

If critical issues are found after release:

1. Create a patched release
   ```bash
   git tag -a v0.1.1 -m "Patch release"
   git push origin v0.1.1
   ```

2. Notify package maintainers
   - Homebrew maintainers
   - Chocolatey maintainers
   - Linux distro maintainers

3. Issue security advisory if needed
   - Post on GitHub security advisories
   - Notify downstream users

## Notes

- Always test releases in clean environments
- Verify code signatures if applicable
- Ensure changelog is comprehensive
- Double-check version numbers match across all files
