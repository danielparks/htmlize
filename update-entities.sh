#!/bin/sh

# Updated entities.json, its license, and the related test data.

set -e

curl -O https://html.spec.whatwg.org/entities.json
curl -Lo entities.json-LICENSE https://github.com/whatwg/html-build/raw/main/LICENSE

<entities.json jq -r 'keys | .[]' > tests/corpus/all-entities-source.txt
<entities.json jq -r 'values | .[].characters' > tests/corpus/all-entities-expanded.txt
