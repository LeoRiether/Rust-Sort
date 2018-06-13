import subprocess
import time
from random import shuffle, randint
import plotly.offline as py
import plotly.graph_objs as go
from numpy import log

def benchmark(n):
  p = subprocess.Popen(['target/debug/quicksort.exe'], stdin=subprocess.PIPE)
  a = [ str(x) for x in list(range(1, n+1)) ]
  shuffle(a)
  d = '\n'.join(a) + '\ne\n' 
  start = time.time()
  p.communicate(input=d.encode())
  end = time.time()
  # print(f'n = {n}; {end-start}s')
  return end-start

if __name__ == '__main__':
  # tests = [1, 1e2, 1e3, 1e4, 1]
  test = 10
  tests = []
  results = []
  t = 0
  n = 8
  while t < 10:
    r = []
    for i in range(n):
      r.append(benchmark(test))
    t = sum(r)/n
    print(f'{test} elements: {t}s')    
    tests.append(test)
    results.append(t)
    test *= 10

  trace = go.Scatter(x=tests, y=results)
  py.plot([ trace ])

  rkey = ''.join([ str(randint(0, 9)) for i in range(0, 4) ])
  with open(f'benchresults{rkey}.csv', 'w') as f:
    f.write('Tests,Results,LogTests,LogResults\n')
    for (test, result) in zip(tests, results):
      f.write(f'{test},{result},{log(test)/log(10)},{log(result)/log(10)}\n')
  