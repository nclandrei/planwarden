#!/usr/bin/env bash
set -euo pipefail

dist_dir="${1:-target/distrib}"
formula_path="${2:-}"

if [[ -z "${formula_path}" ]]; then
  formula_path="$(find "${dist_dir}" -maxdepth 1 -type f -name '*.rb' | head -n 1)"
fi

if [[ -z "${formula_path}" || ! -f "${formula_path}" ]]; then
  echo "could not find Homebrew formula to patch" >&2
  exit 1
fi

shopt -s nullglob
for sha_file in "${dist_dir}"/*.tar.xz.sha256; do
  artifact="$(basename "${sha_file%.sha256}")"
  checksum="$(awk '{print $1; exit}' "${sha_file}")"
  if [[ -z "${checksum}" ]]; then
    echo "missing checksum in ${sha_file}" >&2
    exit 1
  fi

  perl -0pi -e "s@(url \"[^\"]*/\\Q${artifact}\\E\"\\n)(?!\\s+sha256 )@\$1      sha256 \"${checksum}\"\\n@g" "${formula_path}"
done
