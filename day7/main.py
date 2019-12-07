import subprocess
from itertools import permutations

for permutation in permutations([0, 1, 2, 3, 4]):
    print(permutation)

# p = subprocess.Popen(['./target/release/day7'],
#                      stdout=subprocess.PIPE,
#                      stdin=subprocess.PIPE,
#                      stderr=subprocess.PIPE,
#                      shell=True)

# return_value = p.poll()
# while return_value == None:
#     out_line = p.stdout.readline().decode('utf-8').strip()
#     if out_line == 'in>':
#         val = 5
#         print(out_line, val)
#         print(p.communicate(input=str.encode(f"{val}"))[0].decode('utf-8'))
#     else:
#         print(out_line)
#     return_value = p.poll()
