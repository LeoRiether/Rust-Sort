import sys

a = list(range(int(sys.argv[1]), 1, -1))
print('\n'.join([ str(x) for x in a ]))
print('e')