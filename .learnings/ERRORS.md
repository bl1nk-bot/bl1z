# ERRORS LOG

## [ERR-20260602-001] Git Push Authentication Failure

**Recorded at**: 2026-06-02T17:40:00Z
**Priority**: High
**Status**: Resolved
**Area**: infra | config

### Summary
Failed to push changes to remote via HTTPS because terminal prompts are disabled and the PAT was not in the credential store.

### Error
```
fatal: could not read Username for 'https://github.com': terminal prompts disabled
```

### Context
- Command: `git push origin branch`
- Environment: Termux on Android
- Issue: `gh` CLI was logged in, but Git was not configured to use it as a credential helper for write operations.

### Suggested Fix
Run `gh auth refresh -s repo,workflow` to ensure the token has required scopes, and ensure the PAT is used in the remote URL or via `gh auth setup-git`.

### Metadata
- Reproducible: Yes
- Resolved at: 2026-06-02T17:35:00Z
- Solution: Used `git push https://<user>:<token>@github.com/...` after the user refreshed the token.

---
