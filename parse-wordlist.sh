#!/bin/bash
URL='https://kaino.kotus.fi/lataa/nykysuomensanalista2024.csv'
FILE="$(basename $URL)"

if [ ! -f "$FILE" ]; then
    curl "$URL" --output "$FILE"
fi

tail -n +2 "$FILE" | awk '{print $1}' | grep -v '[^a-zA-ZäöåÄÖÅ]' | sed -e '/^$/d' > words.txt

# Add "t" to all words that are easy to conjugate
# The words that are easy to conjugate have third column value 1-5

awk '$3 ~ /^(1|2|3|5|6|8|9|10|11|12|13|15|17|18|19|20|21)$/ {print $1 "t"}' "$FILE" >> words.txt