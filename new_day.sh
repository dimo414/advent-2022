#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$SCRIPT_DIR" || exit

YEAR=2022
COOKIE_JAR=.cookie-jar
URL_FORMAT=https://adventofcode.com/%s/day/%s/input
TMP_FILE=$(mktemp)

if [[ ! -f "$COOKIE_JAR" ]]; then
  {
    echo '# https://curl.se/docs/http-cookies.html'
    printf '%s\t' .adventofcode.com TRUE / FALSE 0 session
  } > "$COOKIE_JAR"
fi

date=${1}
if (( date < 1 || date > 25 )); then
  echo "Invalid date: '${date}'" >&2
  exit 1
fi
printf -v date '%02d' "$date" # add leading 0 if missing
printf -v input_url "${URL_FORMAT}" "$YEAR" "${date#0}" # trim leading zero
dir="src/bin/${date}"

if ! [[ -e "$dir" ]]; then
  mkdir "$dir" && cp src/bin/template/* "$dir" || exit 1
  echo "Created ${dir}:"
else
  echo "${dir} already exists, not overwriting"
fi

if curl --fail --cookie "$COOKIE_JAR" "$input_url" > "$TMP_FILE" 2>/dev/null; then
  mv "$TMP_FILE" "${dir}/input.txt"
elif grep -q $'\t$' "$COOKIE_JAR"; then
  echo "Failed to curl the puzzle input - cookie file seems incomplete!"
  echo "try grabbing the session cookie from a browser and appending it to ${COOKIE_JAR}"
else
  echo "Failed to curl the puzzle input!"
fi

printf 'Run:\tcargo run --bin %s\n' "$date"
printf 'Test:\tcargo test --bin %s\n' "$date"
