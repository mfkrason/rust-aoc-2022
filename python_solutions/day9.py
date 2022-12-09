#!/usr/bin/env python3

import math
import logging
logging.getLogger().setLevel("INFO")

with open('inputs/day9.txt', 'r') as f:
    moves = f.readlines()

def update_tails(given_heads, given_tails):
    x_del = given_heads[0] - given_tails[0]
    y_del = given_heads[1] - given_tails[1]

    logging.debug(f'{given_tails}')
    if abs(x_del) < 2 and abs(y_del) < 2:
        return given_tails

    # To the left or right only
    if abs(x_del) == 2 and y_del == 0:
        logging.debug('Moving 1 left or right')
        x_mod = -1 if x_del < 0 else 1
        new_tails = (given_tails[0] + x_mod, given_tails[1])
        logging.debug(f'\t->{new_tails}')
        return new_tails

    # Above or below only
    if x_del == 0 and abs(y_del) == 2:
        logging.debug('Moving 1 up or down')
        y_mod = -1 if y_del < 0 else 1
        new_tails = (given_tails[0], given_tails[1] + y_mod)
        logging.debug(f'\t->{new_tails}')
        return new_tails

    logging.debug("Moving 1 and 1")
    x_mod = -1 if x_del < 0 else 1
    y_mod = -1 if y_del < 0 else 1
    new_tails = (given_tails[0] + x_mod, given_tails[1] + y_mod)
    logging.debug(f'{new_tails}')
    return new_tails

all_tails = set()
all_tails.add((0,0))

the_knots = [(0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0)]
for move in moves:
    (direction, amount) = move.split(' ')
    logging.debug(f'Moving {move}')
    for i in range(int(amount)):
        match direction:
            case 'R':
                the_knots[0] = (the_knots[0][0] + 1, the_knots[0][1])
            case 'L':
                the_knots[0] = (the_knots[0][0] - 1, the_knots[0][1])
            case 'U':
                the_knots[0] = (the_knots[0][0], the_knots[0][1] + 1)
            case 'D':
                the_knots[0] = (the_knots[0][0], the_knots[0][1] - 1)
        
        for index,knot in enumerate(the_knots):
            if index == 0:
                continue
            the_knots[index] = update_tails(the_knots[index-1], knot)
        all_tails.add(the_knots[-1])
        logging.debug(f'{the_knots[0]} {the_knots[1]}')
print(len(all_tails))
