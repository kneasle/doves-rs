import csv

def load(f):
    with open(f, newline='') as csv_file:
        reader = csv.DictReader(csv_file)
        return list(reader)
