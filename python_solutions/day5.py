#!/usr/bin/env python3
from collections import defaultdict
import re

with open('inputs/day5.txt', 'r') as f:
	lines = f.readlines()

boxes = lines[0:9]
moves = lines[10:]
boxes = [x.replace('   ', '...') for x in boxes]

box_layout = defaultdict(list)
for box in boxes:
	for i in range(0,len(box),4):
		entry = box[i:i+3]
		if '.' not in entry:
			box_layout[str(int(i/4)+1)].append(entry)

for index,stack in box_layout.items():
	stack.reverse()
	box_layout[index] = stack

for move in moves:
	m = re.match("move (\d+) from (\d+) to (\d+)", move)
	if not m:
		raise Exception(f"Bad line: {move}")
	count = m.group(1)
	source = m.group(2)
	dest = m.group(3)
	print(f'{count} {source} {dest}')

	to_move = box_layout[source][-int(count):]
	box_layout[source] = box_layout[source][:-int(count)]
	print(f'moving {to_move}')
	box_layout[dest].extend(to_move)

	"""
	for i in range(int(count)):
		if len(box_layout[source]):
			moving = box_layout[source].pop()
			#print(f'Moving {moving} from {source} to {dest}')
			box_layout[dest].append(moving)
		else:
			print(f"Can't move anything from {box_layout[source]}")
	"""

for k in sorted(box_layout.keys()):
	entry = box_layout[k][-1].replace('[','').replace(']','')
	print(entry, end = '')
print()
