# Experimental Agent Brief for Completing the Retrospect Paper

This document is intended for an AI/software agent that will run the experimental campaign required to complete and strengthen the paper:

**Kubernetes-Native Cloud--Fog--Edge Orchestration for WebAssembly-Enabled Embedded IoT Devices**

Repository under evaluation:

- `https://github.com/lucadagati/k3s-wasm-edge-orchestration`

The agent must produce reproducible measurements, raw records, summary tables, plots, and a concise experimental report that can be transferred into the paper. The goal is not only to show that the system works, but to provide reviewer-grade evidence about correctness, reconciliation semantics, latency, stability, and the impact of controller hardening.

## 1. Ground Rules

- Do not modify the scientific claims of the paper unless the measurements support the change.
- Do not overwrite existing source code without creating a clear patch or branch.
- Record every command needed to reproduce the experiment.
- Save raw outputs before aggregating or plotting them.
- Prefer deterministic scripts over manual steps.
- Every reported number must be traceable to raw trial records.
- Use monotonic timestamps for latency measurements whenever possible.
- Keep separate logs for enrollment, heartbeat, deployment, transactional workflow, and system snapshot.
- Capture the exact repository commit hash and any uncommitted diff before running experiments.

## 2. Required Environment Capture

Before running any experiment, collect the following information and store it in `experiments/<timestamp>/environment/`:

- Repository URL and commit hash:
  - `git remote -v`
  - `git rev-parse HEAD`
  - `git status --short`
  - `git diff --stat`
- Host information:
  - OS distribution and kernel version
  - CPU model and core count
  - RAM size
  - Disk type if available
- Toolchain versions:
  - Docker or container runtime version
  - K3s version
  - kubectl version
  - Rust version
  - Python version
  - Renode version
  - Zephyr SDK/toolchain version if used
  - WAMR version or commit if available
- Kubernetes state:
  - namespaces
  - pods, restarts, readiness
  - services
  - CRDs
  - relevant ConfigMaps/Secrets names, without exposing private key material
- Network setup:
  - TAP interfaces
  - port-forwarding commands
  - DNAT/routing rules

Deliverable: `environment_summary.md` plus raw command outputs.

## 3. Experimental Questions

The campaign must answer the following questions:

1. Can a constrained embedded endpoint enroll securely through the gateway?
2. Does heartbeat supervision remain observable and stable during active sessions?
3. Does application deployment reach a confirmed `Running` phase through the active controller?
4. Does the complete transactional workflow succeed repeatedly?
5. What latency distribution is observed for enrollment, heartbeat observability, deployment, and end-to-end completion?
6. What changes when reconciliation hardening is disabled or bypassed?
7. Are the reported outcomes consistent across gateway logs, Kubernetes CRD status, API statistics, and raw trial records?

## 4. Minimum Campaign Design

Run at least the following campaign:

- Warm-up: 1 complete workflow run, not included in statistics.
- Main campaign: 50 independent trials.
- Optional stronger campaign: 100 independent trials if runtime permits.
- Each trial must include:
  1. runtime prerequisite check;
  2. gateway/API/controller availability check;
  3. device enrollment;
  4. heartbeat freshness verification;
  5. deployment of a minimal WASM application;
  6. confirmation that `Application.status.phase == Running`;
  7. collection of gateway, API, CRD, and trial-level evidence.

A trial is successful only if enrollment, heartbeat supervision, and deployment all succeed in the same trial.

## 5. Metrics to Measure

### 5.1 Enrollment

Measure:

- enrollment completion latency;
- enrollment success/failure;
- time to API visibility;
- gateway response status;
- identity/key match outcome;
- per-stage goodput.

Raw record fields:

```json
{
  "trial_id": 1,
  "stage": "enrollment",
  "start_monotonic_ns": 0,
  "end_monotonic_ns": 0,
  "latency_ms": 0.0,
  "success": true,
  "device_id": "...",
  "gateway_id": "...",
  "api_visible": true,
  "error": null
}
```

### 5.2 Heartbeat Supervision

Measure:

- heartbeat observability latency;
- last heartbeat timestamp;
- heartbeat freshness at verification time;
- gateway-reported connected/disconnected state;
- Kubernetes `Device` phase if available.

Raw record fields:

```json
{
  "trial_id": 1,
  "stage": "heartbeat",
  "latency_ms": 0.0,
  "success": true,
  "gateway_connected": true,
  "last_heartbeat_age_ms": 0.0,
  "device_phase": "Connected",
  "error": null
}
```

### 5.3 Application Deployment

Measure:

- deployment completion latency;
- deployment success/failure;
- time until `Application.status.phase == Running`;
- gateway deployment acknowledgment;
- API statistics as secondary corroboration;
- any stale or conflicting state before convergence.

Raw record fields:

```json
{
  "trial_id": 1,
  "stage": "deployment",
  "latency_ms": 0.0,
  "success": true,
  "application_name": "...",
  "target_device": "...",
  "application_phase": "Running",
  "gateway_ack": true,
  "api_stats_corroborated": true,
  "error": null
}
```

### 5.4 Transactional Workflow

Measure:

- end-to-end latency;
- all-stages success/failure;
- failed stage if any;
- consistency across gateway, controller, and Kubernetes state.

Raw record fields:

```json
{
  "trial_id": 1,
  "stage": "transactional_workflow",
  "latency_ms": 0.0,
  "success": true,
  "enrollment_success": true,
  "heartbeat_success": true,
  "deployment_success": true,
  "failed_stage": null,
  "evidence_consistent": true
}
```

## 6. Statistical Treatment

For each continuous metric report:

- `n` valid trials;
- mean;
- sample standard deviation;
- median;
- minimum;
- maximum;
- p90;
- p95;
- p99;
- interquartile range (IQR);
- coefficient of variation (CV);
- 95% confidence interval for the mean using Student-t.

For each binary success metric report:

- observed success ratio;
- number of successes;
- number of failures;
- 95% Wilson score interval.

For goodput report:

- number of successful trials divided by cumulative stage duration;
- clearly state that this is not cluster-wide saturation throughput.

## 7. Required Ablation: Reconciliation Hardening

The paper claims that operational correctness depends on reconciliation hardening and state ownership. The agent must therefore run an ablation if technically possible.

### 7.1 Hardened Configuration

Run the normal campaign with the current hardened controller behavior.

Expected evidence:

- connected devices are not falsely forced to disconnected state;
- deployment phase converges to `Running`;
- stale CRD status is repaired;
- no mismatch between gateway evidence and Kubernetes resource phase persists after convergence.

### 7.2 Unhardened or Bypassed Configuration

If the repository exposes a prior version, feature flag, branch, or simple patch that disables hardening, run a smaller campaign:

- minimum 10 trials;
- preferred 30 trials.

Measure:

- false disconnection events;
- stale application status;
- empty or missing gateway endpoint propagation;
- divergence between in-memory state and persisted CRD state;
- deployment commands acknowledged by gateway but not converged to `Running`.

If disabling hardening is impossible, produce a documented explanation and collect at least log-level evidence showing where hardening is active in the code path.

Deliverable: `ablation_reconciliation_hardening.md` with before/after comparison.

## 8. Required Output Files

Place all outputs under:

```text
experiments/<YYYYMMDD-HHMMSS>/
```

Required files:

```text
environment/environment_summary.md
environment/*.txt
raw/enrollment_trials.jsonl
raw/heartbeat_trials.jsonl
raw/deployment_trials.jsonl
raw/transactional_trials.jsonl
raw/system_snapshot.json
summary/summary_metrics.json
summary/summary_metrics.csv
summary/latency_table.md
summary/success_table.md
summary/ablation_reconciliation_hardening.md
figures/latency_boxplot.png
figures/latency_cdf.png
figures/success_summary.png        # optional if success rates are not all 1.0
logs/gateway.log
logs/controller.log
logs/api_server.log
logs/kubernetes_resources.yaml
README_REPRODUCE.md
```

## 9. Required Figures

Generate publication-quality figures using the raw trial records.

### 9.1 Latency Boxplot

- Stages: enrollment, heartbeat, deployment, end-to-end.
- Use log scale if heartbeat is much smaller than other stages.
- Show sample means with diamond markers if possible.
- Include `n` in caption.

### 9.2 Empirical CDF

- Include enrollment, deployment, and end-to-end latency.
- Mark p95 and p99 thresholds.
- Explain that CDF means the fraction of trials completed within a given latency threshold.

### 9.3 Optional Ablation Figure

If ablation is possible, generate one compact figure showing:

- false disconnect count before/after hardening;
- stale status count before/after hardening;
- workflow success ratio before/after hardening.

## 10. Evidence Consistency Checks

For every successful trial verify:

- Gateway reports device connected.
- Latest heartbeat is fresh within the configured acceptance window.
- `Application.status.phase` reaches `Running`.
- Gateway deployment acknowledgment exists.
- API statistics do not contradict CRD status.
- Trial record, Kubernetes snapshot, and logs refer to the same device/application identifiers.

Flag any inconsistency in:

```text
summary/evidence_consistency_issues.md
```

## 11. Reviewer-Facing Summary to Produce

Create `summary/reviewer_experimental_summary.md` containing:

1. One paragraph describing the testbed.
2. One paragraph describing the campaign size and procedure.
3. A table of success rates and Wilson intervals.
4. A table of latency metrics.
5. A short statement about reconciliation hardening and ablation results.
6. A limitation statement covering single-node K3s, emulation, and absence/presence of physical hardware.
7. The repository commit hash and path to raw artifacts.

## 12. Acceptance Criteria

The campaign is complete only if:

- all raw records are present;
- all summary tables are generated from raw records;
- all figures are generated from raw records;
- the exact commit hash is recorded;
- success/failure criteria are explicit;
- any failed trials are retained and explained, not discarded;
- the final summary can be pasted into the paper without inventing new numbers.

## 13. Suggested Paper Updates After Measurements

After completing the campaign, update the paper as follows:

- Replace existing latency numbers only if new measurements differ.
- Add or update the reconciliation hardening paragraph with ablation evidence.
- Update the Data Availability and Code Availability statements with the release tag and artifact path.
- Update figures in `paper_ieee_inginf05/figures/` only from generated raw records.
- Recompile the paper and verify page count, references, and figure numbering.