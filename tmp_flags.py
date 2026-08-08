import json,sys
d=json.load(sys.stdin)
t=d['data']['repository']['pullRequest']['reviewThreads']['nodes']
unresolved=[x for x in t if not x['isResolved']]
for x in unresolved:
    c=x['comments']['nodes'][0]
    body=c['body']
    if 'Flag' not in body: continue
    p=c['path']
    lines=[l.strip() for l in body.split(chr(10)) if l.strip() and not l.startswith('<')]
    for l in lines:
        if len(l)>50 and 'Flag' not in l and not l.startswith('P') and not l.startswith('```'):
            print(f'{p}: {l[:200]}')
            break
