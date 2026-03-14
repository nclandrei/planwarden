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
  binary="$(find "${tmpdir}" -type f -name planwarden | head -n 1)"
  if [[ -z "${binary}" || ! -x "${binary}" ]]; then
    echo "could not find extracted planwarden binary in ${tmpdir}" >&2
    exit 1
  fi
fi

"${binary}" --help >/dev/null
"${binary}" schema review roadmap >/dev/null
