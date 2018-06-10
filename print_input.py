from random import shuffle
import sys

a = list(range(1, int(sys.argv[1])+1))
shuffle(a)
for i in a:
  print(i)
print('e')