import sys
from dijkstar import Graph, find_path

input_txt = ""
with open(sys.argv[1], 'r') as f:
  input_txt = f.read()

graph = Graph()
for line in input_txt.splitlines():
  planet, satellite = line.split(')')
  graph.add_edge(planet, satellite, 1)
  graph.add_edge(satellite, planet, 1)

print(find_path(graph, 'YOU', 'SAN'))
print(graph.get_node('SAN'))
