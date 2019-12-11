import sys

input_txt = ""
with open(sys.argv[1], 'r') as f:
    input_txt = f.read()

orbits = {}
for line in input_txt.splitlines():
    planet, satellite = line.split(')')
    if planet in orbits:
        orbits[planet].append(satellite)
    else:
        orbits[planet] = [satellite]

distances_to_com = {}
next_visits = ['COM']
dist = 0
while len(next_visits) > 0:
    tmp_next_visits = []
    while len(next_visits) > 0:
        p = next_visits.pop()
        distances_to_com[p] = dist
        if p in orbits:
            for s in orbits[p]:
                tmp_next_visits.append(s)
    next_visits = tmp_next_visits 
    dist += 1

print('Total:', sum(distances_to_com.values()))
