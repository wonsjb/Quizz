use std::fs;
use yaml_rust::YamlLoader;

use serde::Deserialize;
use csv;

#[derive(Debug, Deserialize)]
pub struct Question {
    pub question: String,
    pub hint: String,
    pub answer: String
}

pub struct Quizz {
    pub title: String,
    pub questions: Vec<Question>,
    pub end_message: String,
}

fn parse_yaml(file: &str) -> Quizz {
    let source = fs::read_to_string(file).expect("Could not read quizz file");
    let docs = YamlLoader::load_from_str(&source).expect("Could not load quizz document");
    let doc = &docs[0];
    let title = String::from(doc["title"].as_str().expect("Could not read title"));
    let mut questions : Vec<Question> = vec!();
    for question_yaml in doc["questions"].as_vec().expect("questions field is not a list") {
        let question = String::from(question_yaml["Q"].as_str().expect("Could not read question"));
        let answer = String::from(question_yaml["A"].as_str().expect("Could not read answer"));
        questions.push(Question{question, answer, hint: String::from("")})
    }
    let end_message = String::from(doc["end_message"].as_str().expect("Could not read end of message"));
    Quizz{title, questions, end_message}
}

fn parse_csv(file: &str) -> Quizz {
    let mut rdr = csv::Reader::from_reader(fs::File::open(file).expect("Could not open file"));
    let mut questions : Vec<Question> = vec!();
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let mut question: Question = result.expect("Could not read question");
        question.question = question.question.replace("\\n", "\n");
        questions.push(question);
    }
    Quizz{title: String::from("Quizz!"), questions, end_message: String::from("Thanks\nFor\nPlaying")}
}


impl Quizz {

    pub fn parse(file: &str) -> Self {
        if file.ends_with(".yaml") {
            parse_yaml(file)
        } else if file.ends_with(".csv") {
            parse_csv(file)
        } else {
            panic!("Unknown format for file {}", file)
        }
    }

    pub fn get_texts(&self) -> Vec<(String, String)> {
        let mut to_display : Vec<(String, String)> = vec![(String::from(""), self.title.clone())];
        let mut count = 1;
        for question in &self.questions {
            let mut question_str = question.question.clone();
            if question.hint.len() > 0 {
                question_str.push_str(&format!("\n({})", question.hint));
            }
            to_display.push((format!("Question {}/{}:", count, self.questions.len()), question_str));
            to_display.push((format!("Answer {}/{}:", count, self.questions.len()), question.answer.clone()));
            count = count + 1;
        }
        to_display.push((String::from(""), self.end_message.clone()));
        to_display
    }
}