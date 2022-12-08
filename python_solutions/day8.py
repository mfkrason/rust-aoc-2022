#!/usr/bin/env python3

with open('inputs/day8-stell.txt', 'r') as f:
    lines = f.readlines()

lines = [x.strip() for x in lines]
trees = []
for line in lines:
    trees.append([x for x in line])


def part1():
    visible_trees = set()
    for rowi,row in enumerate(trees):
        for coli,col in enumerate(row):
            if rowi == 0 or rowi == len(trees)-1:
                visible_trees.add((rowi,coli))
                continue
            if coli == 0 or coli == len(row)-1:
                visible_trees.add((rowi,coli))
                continue

            left = max(row[:coli])
            right = max(row[coli+1:])
            above = max([x[coli] for x in trees[:rowi]])
            below = max([x[coli] for x in trees[rowi+1:]])

            if col > left or col > right or col > above or col > below:
                #print(f'{col} @ {(rowi,coli)} is visible')
                visible_trees.add((rowi,coli))
    print(visible_trees)
    print(len(visible_trees))

def find_distance_to_blocker(data, value):
    for i, test in enumerate(data):
        if test >= value:
            return i+1
    return len(data)

def part2():
    tree_values = []
    max_value = 0
    for rowi,row in enumerate(trees):
        tree_values.append([])
        for coli,col in enumerate(row):
            left = find_distance_to_blocker(row[:coli][::-1], col)
            right = find_distance_to_blocker(row[coli+1:], col)
            above = find_distance_to_blocker([x[coli] for x in trees[:rowi]][::-1], col)
            below = find_distance_to_blocker([x[coli] for x in trees[rowi+1:]], col)

            tree_values[rowi].append(left*right*above*below)
        max_value = max([max_value, max(tree_values[rowi])])

    print(max_value)

part1()
            



            
            
