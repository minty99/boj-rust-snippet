use ps_rust::solve;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use std::error::Error;
use std::io::{BufReader, BufWriter, Cursor};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let problem_id: usize = args[1].parse()?;
        eprintln!("Testing BOJ {problem_id}...");
        run_boj_sample(problem_id)
    } else {
        ps_rust::main()
    }
}

fn run_boj_sample(problem_id: usize) -> Result<(), Box<dyn Error>> {
    let url = format!("https://www.acmicpc.net/problem/{}", problem_id);

    // Fetch the webpage content
    let client = Client::new();
    let body = client
            .get(url)
            .header(USER_AGENT, "Mozilla/5.0 (Window NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
            .send()?
            .text()?;

    // Parse the HTML
    let document = Html::parse_document(&body);

    // Prepare to collect inputs and outputs
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    // Base selectors for example inputs and outputs
    let input_selector = Selector::parse("pre[id^='sample-input-']").unwrap();
    let output_selector = Selector::parse("pre[id^='sample-output-']").unwrap();

    // Collect inputs
    for element in document.select(&input_selector) {
        inputs.push(element.text().collect::<Vec<_>>().join("\n"));
    }

    // Collect outputs
    for element in document.select(&output_selector) {
        outputs.push(element.text().collect::<Vec<_>>().join("\n"));
    }

    eprintln!("{} samples found\n", inputs.len());

    for (idx, (input, answer)) in inputs.into_iter().zip(outputs.into_iter()).enumerate() {
        let input_bytes = input.into_bytes();
        let mut output_bytes = Vec::new();

        // Run the solution
        solve(
            BufReader::new(Cursor::new(input_bytes)),
            BufWriter::new(&mut output_bytes),
        )?;
        let output = String::from_utf8(output_bytes)?;

        // Compare answer
        if answer.trim_end() == output.trim_end() {
            eprintln!("Case #{}: Correct", idx + 1);
        } else {
            eprintln!(
                "Case #{}: Wrong Answer\nanswer:\n{}\n\noutput:\n{}\n",
                idx + 1,
                answer.trim_end(),
                output.trim_end(),
            );
            return Err(format!("Wrong Answer on test {}", idx + 1).into());
        }
    }

    Ok(())
}
