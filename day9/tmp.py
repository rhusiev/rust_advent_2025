from itertools import combinations, pairwise


sort = lambda l: [((min(a,c),min(b,d)),(max(a,c),
                 max(b,d))) for (a,b),(c,d) in l]

red = list(map(eval, open('input.txt')))
green = sort(pairwise(red + [red[0]]))

a = b = 0

for (x,y), (u,v) in sort(combinations(red, 2)):
    size = (u-x+1) * (v-y+1)

    if size > a: a = size

    if size > b:
        for (p,q), (r,s) in green:
            if p<u and q<v and r>x and s>y: break

        else: b = size

print(a, b)
