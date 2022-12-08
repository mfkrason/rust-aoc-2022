#!/usr/bin/env python3

with open('inputs/day4.txt', 'r') as f:
    lines = f.readlines()

assignment = []

counter = 0
for line in lines:
    (f,t) = line.strip().split(',')
    f = f.split('-')
    f = {x for x in range(int(f[0]), int(f[1])+1)}

    t = t.split('-')
    t = {x for x in range(int(t[0]), int(t[1])+1)}

    if len(f.intersection(t)):
        counter += 1

print(counter)
