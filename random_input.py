from random import randint
import sys

r = 0
if len(sys.argv) >= 3:
  r = int(sys.argv[2])
else:
  r = int(sys.argv[1])

for i in range(int(sys.argv[1])):
  print(randint(1, r))
print('e')