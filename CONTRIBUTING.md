# Contributing — Red Hat Vector Fork

This guide covers what is specific to contributing to this fork. For generic Vector development workflow, coding standards, and testing, see the [upstream CONTRIBUTING.md](https://github.com/vectordotdev/vector/blob/master/CONTRIBUTING.md).

## Branching Strategy

| Branch       | Purpose                                         |
|--------------|-------------------------------------------------|
| `rh-main`    | Documentation only — **never commit code here** |
| `v0.47.0-rh` | Stable development — OpenShift 6.2–6.5          |
| `v0.54.0-rh` | Next-gen development — future releases          |

## Submitting Changes

1. Check out the target version branch: `git checkout v0.47.0-rh`
2. Create a feature branch: `git checkout -b LOG-XXXX-short-description`
3. Make changes, test with `make test`
4. Push to your fork and open a PR against the version branch (not `rh-main`)
5. Include the Jira ticket ID (LOG-XXXX) in the PR title and description

If a fix applies to multiple version branches, submit a separate PR for each.

## Commit Messages

Use the Jira ticket ID and conventional commit format:

```
fix(file-source): handle symlink rotation during log collection

Fixes LOG-7506
```

PR titles follow upstreams [conventional commits](https://www.conventionalcommits.org) format:
```text
feat(scope): description
fix(scope): description
chore(scope): description
```
or by adding Jira Issue Key like:
```text
LOG-7506: description
```

## Code Review

Reviews follow the [OWNERS](OWNERS) file:

- **Approvers:** jcantrill, alanconway, xperimental
- **Reviewers:** jcantrill, Clee2691, vparfonov, cahartma

PRs require at least one approval. Significant changes require two approvers.

## Red Hat-Specific Code

When adding patches that diverge from upstream:

- Reference the Jira ticket in code comments explaining why
- If the feature could be upstreamed, note it in the PR description
- Update [ARCHITECTURE.md](ARCHITECTURE.md) if the change affects design decisions or the `ocp-logging` feature scope

## Contributing Back to Upstream

We minimize fork divergence by upstreaming changes when possible:

1. Implement and test the fix in this fork first
2. If the fix is generic (not OpenShift-specific), propose it to [upstream Vector](https://github.com/vectordotdev/vector)
3. Once accepted upstream, simplify or remove the fork patch on the next upstream merge

## Filing Issues

File issues at [Red Hat JIRA](https://issues.redhat.com) under the **LOG** project. Include [must-gather](https://github.com/openshift/cluster-logging-operator/tree/master/must-gather) output for debugging.
