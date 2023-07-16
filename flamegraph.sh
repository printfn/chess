#!/usr/bin/env bash
set -euo pipefail

RAYON_NUM_THREADS=1 PERF=/usr/lib/linux-tools/5.15.0-76-generic/perf CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --unit-test chess-core -- benchmark
