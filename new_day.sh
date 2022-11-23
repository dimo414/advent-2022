#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$SCRIPT_DIR" || exit

COOKIE_JAR=.cookie-jar
URL_FORMAT=https://adventofcode.com/2022/day/%s/input
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
printf -v date '%02d' "$date" # add leading 0
printf -v input_url "${URL_FORMAT}" "${date}"
dir="src/bin/${date}"

mkdir "$dir" || exit 1
cp src/bin/template/* "$dir"

if curl --fail --cookie "$COOKIE_JAR" "$input_url" > "$TMP_FILE" 2>/dev/null; then
  mv "$TMP_FILE" "${dir}/input.txt"
else
  echo "Failed to curl the puzzle input!"
  echo "try grabbing the session cookie from a browser and appending it to ${COOKIE_JAR}"
fi

echo "Created ${dir}:"
printf 'Run:\tcargo run --bin %s\n' "$date"
printf 'Test:\tcargo test --bin %s\n' "$date"
