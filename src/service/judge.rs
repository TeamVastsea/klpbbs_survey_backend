use std::collections::HashMap;
use std::str::FromStr;
use futures::executor::block_on;
use tracing::debug;
use uuid::Uuid;
use crate::model::question::{Answer, Question, QuestionType};
use crate::model::ValueWithTitle;

pub async fn judge_subjectives(questions: Vec<Question>, answer: HashMap<String, String>) -> (i32, HashMap<Uuid, i32>) {
    let mut score = HashMap::new();
    let mut full_score = 0;
    
    for question in questions {
        if let Some(ans) = question.answer { 
            full_score += ans.all_points;
            
            match question.r#type {
                QuestionType::MultipleChoice => {
                    let point = judge_multiple_choice(&ans.answer, answer.get(&question.id.to_string()).unwrap());
                    match point {
                        PointType::All => {
                            score.insert(question.id, ans.all_points);
                        }
                        PointType::Sub => {
                            if let Some(sub) = ans.sub_points {
                                score.insert(question.id, sub);
                            }
                        }
                        PointType::None => {
                            score.insert(question.id, 0);

                        }
                    }
                }
                QuestionType::SingleChoice => {
                    let point = judge_single_choice(&ans.answer, answer.get(&question.id.to_string()).unwrap());
                    match point {
                        PointType::All => {
                            score.insert(question.id, ans.all_points);
                        }
                        PointType::None => {
                            score.insert(question.id, 0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    (full_score, score)
}

pub fn judge_multiple_choice(answer: &str, user: &str) -> PointType {
    let answer: Vec<String> = serde_json::from_str(answer).unwrap();
    let user: Vec<String> = serde_json::from_str(user).unwrap();
    debug!("answer: {:?}, user: {:?}", answer, user);
    let mut missing_flag = false;
    let mut wrong_flag = false;
    
    for ans in &answer {
        if !user.contains(ans) {
            missing_flag = true;
            break;
        }
    }

    for u in &user {
        if !answer.contains(u) {
            wrong_flag = true;
            break;
        }
    }
    
    if wrong_flag {
        PointType::None
    } else if missing_flag {
        PointType::Sub
    } else {
        PointType::All
    }
}

pub fn judge_single_choice(answer: &str, user: &str) -> PointType {
    if answer == user {
        PointType::All
    } else {
        PointType::None
    }
}

pub enum PointType {
    All,
    Sub,
    None,
}

#[test]
fn test_judge() {
    let question1 = Question {
        id: Uuid::from_str("d4135fda-00f3-4dd0-a19b-52150322912f").unwrap(),
        content: ValueWithTitle {
            title: "What is the capital of China?".to_string(),
            content: "What is the capital of China?".to_string(),
        },
        r#type: QuestionType::SingleChoice,
        values: Some(vec![
            ValueWithTitle {
                title: "Beijing".to_string(),
                content: "Beijing".to_string(),
            },
            ValueWithTitle {
                title: "Shanghai".to_string(),
                content: "Shanghai".to_string(),
            },
            ValueWithTitle {
                title: "Guangzhou".to_string(),
                content: "Guangzhou".to_string(),
            },
            ValueWithTitle {
                title: "Shenzhen".to_string(),
                content: "Shenzhen".to_string(),
            },
        ]),
        condition: None,
        required: true,
        answer: Some(Answer {
            answer: "0".to_string(),
            all_points: 10,
            sub_points: None,
        }),
    };

    let question2 = Question {
        id: Uuid::from_str("4135f52b-39a1-4aca-a19c-498ccb879725").unwrap(),
        content: ValueWithTitle {
            title: "Select A and B".to_string(),
            content: "Hello!".to_string(),
        },
        r#type: QuestionType::MultipleChoice,
        values: Some(vec![
            ValueWithTitle {
                title: "A".to_string(),
                content: "A".to_string(),
            },
            ValueWithTitle {
                title: "B".to_string(),
                content: "B".to_string(),
            },
            ValueWithTitle {
                title: "C".to_string(),
                content: "C".to_string(),
            },
            ValueWithTitle {
                title: "D".to_string(),
                content: "D".to_string(),
            },
        ]),
        condition: None,
        required: false,
        answer: Some(Answer {
            answer: "[\"0\",\"1\"]".to_string(),
            all_points: 5,
            sub_points: Some(3),
        }),
    };

    let questions = vec![question1, question2];
    let mut answer = HashMap::new();
    answer.insert("d4135fda-00f3-4dd0-a19b-52150322912f".to_string(), "0".to_string());
    answer.insert("4135f52b-39a1-4aca-a19c-498ccb879725".to_string(), "[\"0\"]".to_string());

    let (full, scores) = block_on(judge_subjectives(questions, answer));

    println!("{} {:?}", full, scores);
    
    assert_eq!(full, 15);
    assert_eq!(scores.get(&Uuid::from_str("d4135fda-00f3-4dd0-a19b-52150322912f").unwrap()), Some(&10));
    assert_eq!(scores.get(&Uuid::from_str("4135f52b-39a1-4aca-a19c-498ccb879725").unwrap()), Some(&3));
}