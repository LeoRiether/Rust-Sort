from random import shuffle
import sys

with open(sys.argv[1], 'w') as f:
  a = list(range(1, int(sys.argv[2])))
  shuffle(a)
  f.write('\n'.join([ str(x) for x in a ]))
  f.write('\ne\n')