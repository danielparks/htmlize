#!/bin/sh

# Updated entities.json and the related test.

set -e

curl -O https://html.spec.whatwg.org/entities.json

<entities.json jq -r 'keys | .[]' > tests/corpus/all-entities-source.txt
<entities.json jq -r 'values | .[].characters' > tests/corpus/all-entities-expanded.txt
