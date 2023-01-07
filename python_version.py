import re
import itertools
import os
import math

def get_num_dorks(place_holder_list):
    lengths = [len(open(f"{place_holder}.txt", 'r').read().splitlines()) for place_holder in place_holder_list]
    return math.prod(lengths)

def main():
    with open("DorkTypes.txt", 'r') as f:
        dork_types = f.readlines()

    place_holder_list = []
    pattern1 = re.compile(r'\((.*?)\)')
    for dork_type in dork_types:
        matches = pattern1.findall(dork_type)
        for match in matches:
            if match not in place_holder_list:
                place_holder_list.append(match)
    num_dorks = get_num_dorks(place_holder_list)
    num_dorks_str = "{:,}".format(num_dorks)
    print(f"Number of dorks: {num_dorks_str}")

    with open("combination.txt", "a") as f:
        for dork_type in dork_types:
            combinations = itertools.product(*[open(f"{place_holder}.txt", 'r').read().splitlines() for place_holder in place_holder_list])

            for combination in combinations:
                modified_dork_type = dork_type.strip()
                for j, place_holder in enumerate(place_holder_list):
                    pattern = re.compile(f'\({place_holder}\)')
                    modified_dork_type = pattern.sub(combination[j], modified_dork_type)
                f.write(f"{modified_dork_type}\n")

if __name__ == "__main__":
    if "combination.txt" in os.listdir():
        os.remove("combination.txt")
    main()
