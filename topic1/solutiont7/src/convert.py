import json

def convert_file_to_json(input_file: str, output_file: str) -> None:
    data = { }

    with open(input_file, 'r', encoding='utf-8') as infile:
        for line in infile:
            parts = line.strip().split()

            if len(parts) != 2:
                print(f"SKIP: {parts}")
                continue

            data[parts[0].strip()] = parts[1].strip().rstrip("*")

    with open(output_file, 'w', encoding='utf-8') as outfile:
        json.dump(data, outfile, ensure_ascii=False, indent=4)

if __name__ == "__main__":
    input_file = "../administrative_division_code.txt"
    output_file = "../region.json"
    convert_file_to_json(input_file, output_file)
