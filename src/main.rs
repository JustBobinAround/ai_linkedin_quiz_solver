extern crate openai_api;
use openai_api::prelude::*;
use std::net::{TcpListener, TcpStream};
use scraper::{ Html, Selector };
use std::thread::spawn;
use tungstenite::accept;

#[macro_export]
macro_rules! nested_unwrap {
    ( $mutex:ident ; $code:block ) => {
        if let Ok($mutex) = $mutex {
            $code;
        };
    };
    ( $head:ident, $($tail:ident),* ; $code:block ) => {
        if let Ok($head) = $head {
            nested_unwrap!($($tail),* ; $code);
        };
    };
}

fn handle_request(stream: TcpStream) {
    if let Ok(mut websocket) = accept(stream) {
        if let Ok(msg) = websocket.read() {
            if msg.is_binary() || msg.is_text() {
                let document = Html::parse_document(&format!("{}",msg));
                let mut prompt: String = String::from("Question:\n");
                let quiz_title = Selector::parse("header.sa-assessment-flow__header");
                let quiz_inner = Selector::parse("h1.text-align-center");
                let question = Selector::parse("span.sa-assessment-quiz__multi-line");
                let question_detail = Selector::parse("div.sa-assessment-quiz__title-detail");
                let choice_span = Selector::parse("span.sa-question-basic-multichoice__multiline");
                let choice = Selector::parse("span.visually-hidden");

                nested_unwrap!(quiz_title, quiz_inner,question, question_detail, choice_span, choice; {
                document
                    .select(&quiz_title)
                    .for_each(|element|{
                        if let Some(option) = element.select(&quiz_inner).next() {
                            let mut inner_sum = String::new();
                            for item in option.text() {
                                inner_sum = format!("{}{}", inner_sum, item);
                            }
                            prompt = format!("{}{}\n\n", prompt, inner_sum);
                        }
                    }); 
                document
                    .select(&question)
                    .for_each(|element|{
                        if let Some(option) = element.select(&choice).next() {
                            let mut inner_sum = String::new();
                            for item in option.text() {
                                inner_sum = format!("{}{}", inner_sum, item);
                            }
                            prompt = format!("{}{}\n\n", prompt, inner_sum);
                        }
                    }); 

                document
                    .select(&question_detail)
                    .for_each(|element|{
                        if let Some(option) = element.select(&choice).next() {
                            let mut inner_sum = String::new();
                            for item in option.text() {
                                inner_sum = format!("{}{}", inner_sum, item);
                            }
                            prompt = format!("{}{}\n\n", prompt, inner_sum);
                        }
                    });

                document
                    .select(&choice_span)
                    .zip('a'..='z')
                    .for_each(|(element,c)|{
                        if let Some(option) = element.select(&choice).next() {
                            let mut inner_sum = String::new();
                            for item in option.text() {
                                inner_sum = format!("{}{}", inner_sum, item);
                            }
                            prompt = format!("{}Option {}:\n{}\n\n", prompt,c, inner_sum);
                        }
                    }); 

                println!("PROMPT:\n{}", prompt);
                let request = gpt35![
                    system!("You goal is to answer the following question by choosing one of the options:"),
                    user!(prompt)
                ];

                if let Ok(response) = request.get() {
                    println!("RESPONSE:\n{}", response.default_choice());
                    let _ = websocket.send(tungstenite::Message::Text(response.default_choice()));
                    let _ = websocket.send(msg);
                } else {
                    let _ = websocket.send(tungstenite::Message::Text("failed to get gpt35 response".to_owned()));
                }
                

                });
            }
        }
    }
}

fn main () {
    let addr = "127.0.0.1:9001";
    let server = TcpListener::bind(addr)
        .expect(&format!("Failed to open socket at:{}", addr));
    println!("Listening on: {}", addr);
    server
        .incoming()
        .filter_map(|stream| stream.ok())
        .for_each(|stream| {
            spawn(move || {handle_request(stream)});
        });
}

