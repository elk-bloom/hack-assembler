use std::fs;
use std::path::Path;
use std::process::Command;

struct TestResult {
    file: String,
    differences: String,
}

fn compare_outputs(expected: &str, actual: &str) -> Option<String> {
    let mut differences = String::new();
    let mut different = false;

    let expected_lines = expected.lines();
    let actual_lines = actual.lines();
    let expected_lines_count = expected_lines.clone().count();
    let actual_lines_count = actual_lines.clone().count();

    for (line_num, (expected_line, actual_line)) in expected_lines.zip(actual_lines).enumerate() {
        if expected_line != actual_line {
            different = true;
            let vertical_difference_string: String = expected_line
                .chars()
                .zip(actual_line.chars())
                .map(|(expected_char, actual_char)| {
                    if expected_char == actual_char {
                        ' '
                    } else {
                        '^'
                    }
                })
                .collect();

            differences.push_str(&format!(
                "Difference at line {}:\nExpected:\t{}\nActual:\t\t{}\n\t\t{}\n",
                line_num + 1,
                expected_line,
                actual_line,
                vertical_difference_string
            ));
        }
    }

    if expected_lines_count != actual_lines_count {
        different = true;
        differences.push_str(&format!(
            "Expected {} lines in output, but got {} lines\n",
            expected_lines_count, actual_lines_count
        ));
    }

    if different {
        Some(differences)
    } else {
        None
    }
}

#[test]
fn test_assembler_output() {
    let input_dir = Path::new("tests/inputs");
    let output_dir = Path::new("tests/outputs");
    let expected_output_dir = Path::new("tests/expected_outputs");
    let mut test_results: Vec<TestResult> = Vec::new();

    for entry in fs::read_dir(input_dir).expect("Failed to read input directory") {
        let entry = entry.expect("Failed to read input file");
        let asm_input_path = entry.path();

        let stem = asm_input_path.file_stem().expect("Failed to get file stem");
        let hack_output_path = output_dir.join(stem).with_extension("hack");
        let expected_hack_output_path = expected_output_dir.join(stem).with_extension("hack");
        let hack_file_name = expected_hack_output_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(&asm_input_path)
            .arg("--output")
            .arg(&hack_output_path)
            .status()
            .expect("Failed to run assembler");

        let actual_output =
            fs::read_to_string(hack_output_path).expect("Failed to read actual output file");
        let expected_output = fs::read_to_string(&expected_hack_output_path)
            .expect("Failed to read expected output file");

        if let Some(differences) = compare_outputs(&expected_output, &actual_output) {
            test_results.push(TestResult {
                file: hack_file_name,
                differences,
            });
        }
    }

    if !test_results.is_empty() {
        let mut error_message = String::from("Test failed for the following files:\n");
        for result in test_results {
            error_message.push_str(&format!(
                "File '{}':\n{}\n",
                result.file, result.differences
            ));
        }
        panic!("{}", error_message);
    }
}
