import re
import os
import sys

def check_file(file_path):
    with open(file_path, 'r') as f:
        c = f.read()

    if re.search(rf'{sys.argv[1]}', c):
        print(f'match {file_path}')



for root, dirs, files in os.walk('.'):
    for file in files:
        if file.endswith(".rs"):
            check_file(os.path.join(root, file))
