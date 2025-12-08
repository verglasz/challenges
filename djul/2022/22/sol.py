import requests
import json

endpoint = "https://cparta.djul.datasektionen.se/graphql"

pondsquery = "{countries { ponds {ducks {id}}}}"


def ask(q):
    res = requests.post(endpoint, json={"query": q})
    if not res.ok:
        print(res.content)
    body = json.loads(res.content)
    return body['data']


ducks: set[str] = set()
nextducks: set[str] = set()

base = ask(pondsquery)
print(base)
for country in base['countries']:
    for pond in country['ponds']:
        theducks = pond['ducks']
        nextducks.update(d['id'] for d in theducks)

while len(nextducks) > 0:
    nextnext = set()
    for did in nextducks:
        query = r'{duck(id: "' + did + r'") {family {id ... on EvilDuck {flag} } } }'
        print(query)
        family = ask(query)
        for duck in family['duck']['family']:
            if 'flag' in duck:
                print(f"found flag!: `{duck['flag']}`")
            theid = duck['id']
            if theid in ducks or theid in nextducks:
                continue
            nextnext.add(theid)
    ducks.update(nextducks)
    nextducks = nextnext

print('exhausted ducks')
