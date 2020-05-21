use std::fs;
use yaml_rust::YamlLoader;

pub struct Question {
    pub question: String,
    pub answer: String
}

pub struct Quizz {
    pub title: String,
    pub questions: Vec<Question>,
    pub end_message: String,
}

impl Quizz {
    pub fn parse(file: &str) -> Self {
        let source = fs::read_to_string(file).expect("Could not read quizz file");
        let docs = YamlLoader::load_from_str(&source).expect("Could not load quizz document");
        let doc = &docs[0];
        let title = String::from(doc["title"].as_str().expect("Could not read title"));
        let mut questions : Vec<Question> = vec!();
        for question_yaml in doc["questions"].as_vec().expect("questions field is not a list") {
            let question = String::from(question_yaml["Q"].as_str().expect("Could not read question"));
            let answer = String::from(question_yaml["A"].as_str().expect("Could not read answer"));
            questions.push(Question{question, answer})
        }
        let end_message = String::from(doc["end_message"].as_str().expect("Could not read end of message"));
        Quizz{title, questions, end_message}
    }

    pub fn get_texts(&self) -> Vec<(String, String)> {
        let mut to_display : Vec<(String, String)> = vec![(String::from(""), self.title.clone())];
        let mut count = 1;
        for question in &self.questions {
            to_display.push((format!("Question {}/{}:", count, self.questions.len()), question.question.clone()));
            to_display.push((format!("Answer {}/{}:", count, self.questions.len()), question.answer.clone()));
            count = count + 1;
        }
        to_display.push((String::from(""), self.end_message.clone()));
        to_display
    }
}