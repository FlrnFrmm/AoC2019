from math import pi, atan2, degrees 

class Asteroid:

    def __init__(self, x, y):
        self.x = x
        self.y = y

    def radians_to(self, other):
        (d_x, d_y) = other.x - self.x, other.y - self.y
        rad = atan2(d_y, d_x)
        return rad + pi / 2 if rad >= 0 else rad + 1.5 * pi

asteroids = []
with open('input.txt', 'r+') as f:
    content = f.read()
    for i, l in enumerate(content.split('\n')):
        for j, c in enumerate(l):
            if c is '#':
                asteroids.append(Asteroid(j,i))

most_in_sight = 0
best_asteroid = None
for (i, asteroid) in enumerate(asteroids):
    in_sight = []
    for (j, other_asteroid) in enumerate(asteroids):
        if i != j:
            radians = asteroid.radians_to(other_asteroid)
            r_key = int(str(radians).replace('.',''))
            if radians not in in_sight:
                in_sight.append(radians)
    if len(in_sight) > most_in_sight:
        most_in_sight = len(in_sight)
        best_asteroid = asteroid

print("({},{}) -> {}".format(best_asteroid.x, best_asteroid.y, most_in_sight))                

one = Asteroid(1,1)
two = Asteroid(0.5,2)
print(one.radians_to(two), pi + 0.5 * pi)

from math import atan2,pi

with open("input.txt","rt") as f: t = f.read().strip().splitlines()
xy = {(x,y) for y,l in enumerate(t) for x,c in enumerate(l) if c=='#'}

def angle(x,y): return round((atan2(x,y)+2*pi)%(2*pi),10)

n = {a:len({angle(e[0]-a[0],a[1]-e[1]) for e in xy if a!=e}) for a in xy}
m = max(n.values())
print( m ) # 278

a = [k for k,v in n.items() if v==m][0] # (23, 19)

dirs = {} # {angle -> [dist -> (x,y), ...]}
for e in xy:
  if a!=e:
    dx = e[0]-a[0]; dy = a[1]-e[1]
    dirs.setdefault(angle(dx,dy),[]).append((dx**2+dy**2,e))

def remove_and_count( dd ):
  idx = 0 # index of item being removed
  while 'do several times':
    for d in dd:
      if len(d[1])>0:
        idx += 1; a = d[1].pop(0)
        if idx==200: # looking for the 200th item
          return a[1][0]*100+a[1][1] # 1417
print( remove_and_count( (k,sorted(v)) for k,v in sorted(dirs.items()) ) )