#!/usr/bin/env bash
set -euo pipefail

binary="${1:?usage: smoke-test-installed-planwarden.sh /path/to/planwarden-or-tarball}"
tmpdir=""

cleanup() {
  if [[ -n "${tmpdir}" && -d "${tmpdir}" ]]; then
    rm -rf "${tmpdir}"
  fi
}
trap cleanup EXIT

if [[ -f "${binary}" && "${binary}" == *.tar.xz ]]; then
  tmpdir="$(mktemp -d)"
  tar -xJf "${binary}" -C "${tmpdir}"
  binary="${tmpdir}/planwarden"
fi

"${binary}" --help >/dev/null
"${binary}" schema review roadmap >/dev/null
