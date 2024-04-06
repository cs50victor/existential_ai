use futures::StreamExt;
use oogway::{ask_helper, Oogway};
use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let master_oogway = Oogway::new()?;

    let mut question =
        "Hello, welcome to Wendy's! Thank you for choosing us today. How can I assist you?"
            .to_string();

    print!("\n> You :\n\t{question}");

    // for dev / testing
    let use_stream = true;

    loop {
        let mut lock = stdout().lock();

        print!("\n> Oogway : \n\t");

        if use_stream {
            // let mut response_stream = ask_helper(master_oogway, question) master_oogway.ask(question.clone()).await?;
            let mut response_stream = ask_helper(master_oogway.clone(), question.clone()).await?;

            while let Some(result) = response_stream.next().await {
                match result {
                    Ok(response) => {
                        response.choices.iter().for_each(|chat_choice| {
                            if let Some(ref content) = chat_choice.delta.content {
                                write!(lock, "{}", content).unwrap();
                            }
                        });
                    },
                    Err(err) => {
                        writeln!(lock, "error: {err}").unwrap();
                    },
                }
                stdout().flush()?;
            }
        } else {
            let response = master_oogway.ask_and_wait(question.clone()).await?;
            for choice in response.choices {
                write!(lock, "{}", choice.message.content.unwrap_or_default()).unwrap();
            }
        }
        get_input(&mut question, "\n> You: \t ");
        println!("curr question - {question}");
    }
}

pub fn get_input(input: &mut String, prompt: &str){
    println!("{}", prompt);
    input.clear();
    match stdin().read_line(input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    *input = input.trim().to_string();
}
