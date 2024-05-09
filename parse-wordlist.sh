#!/bin/bash
URL='https://kaino.kotus.fi/lataa/nykysuomensanalista2024.csv'
FILE="$(basename $URL)"

if [ ! -f "$FILE" ]; then
    curl "$URL" --output "$FILE"
fi

# Extract the first column, remove header line, remove words with non-ASCII characters, no trailing empty line
awk '{print $1}' "$FILE" | tail -n +2 | grep -v '[^a-zA-ZäöåÄÖÅ]' | sed -e '/^$/d' > words.txt