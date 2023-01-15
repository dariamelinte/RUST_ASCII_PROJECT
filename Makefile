run_json:
	cargo run -- --inputFile "./example.json" --outputFile "./result.txt"

run_csv:
	cargo run -- --inputFile "./example.csv" --outputFile "./result.txt"

run_separated: 
	cargo run -- --inputFile "./example.json" --outputFile "./result.txt" --separated

run_align_center: 
	cargo run -- --inputFile "./example.json" --outputFile "./result.txt" --separated --alignment "center"

run_align_right: 
	cargo run -- --inputFile "./example.json" --outputFile "./result.txt" --alignment "right"