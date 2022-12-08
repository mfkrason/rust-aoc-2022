#!/usr/bin/env python3

with open('inputs/day7.txt', 'r') as f:
    lines = f.readlines()

class Directory:
    def __init__(self, name, parent):
        self.name = name
        self.parent = parent
        self.directories = {}
        self.files = []
        self.size = 0

    def add_directory(self, directory):
        self.directories[directory] = Directory(directory, self)

    def add_files(self, name, size):
        self.files.append((name, size))

    def get_size(self):
        my_size = sum([x[1] for x in self.files])
        directory_sizes = sum([directory.get_size() for name,directory in self.directories.items()])

        total_size = my_size + directory_sizes

        self.size = total_size

        return total_size

    def part1(self, nodes):

        if self.size <= 100000:
            nodes.append(self)

        for name,directory in self.directories.items():
            directory.part1(nodes)

        return nodes

    def part2(self, nodes, needed_size):

        if self.size >= needed_size:
            nodes.append(self)

        for name,directory in self.directories.items():
            directory.part2(nodes, needed_size)

        return nodes

    def __str__(self):
        return f'{self.name} {self.directories} {self.files}'

TOTAL_DISK=70000000
NEEDED=30000000

root = Directory('/', None)
current_directory = root

for line in lines[1:]:
    line = line.strip()
    if line.startswith('$'):
        cmd = line.split(' ')[1:]

        if cmd[0] == 'cd':
            if cmd[1] == '..':
                current_directory = current_directory.parent
            else:
                current_directory = current_directory.directories[cmd[1]]

        if cmd[0] == 'ls':
            continue
    else:
        (info, name) = line.split(' ')
        if 'dir' in info:
            current_directory.add_directory(name)
        else:
            current_directory.add_files(name, int(info))

print( root.get_size() )
under10k = []
root.part1(under10k)
print(sum([x.size for x in under10k]))

current_unused = TOTAL_DISK - root.size
current_needed = NEEDED - current_unused

big_enough = []
root.part2(big_enough, current_needed)
print(f'{current_unused} {current_needed}')
big_enough = sorted(big_enough, key=lambda x: x.size)
print(f'{[x.name for x in big_enough]}')
print(f'{big_enough[0].size}')


