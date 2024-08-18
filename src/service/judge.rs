use std::collections::HashMap;
use tracing::debug;
use crate::model::question::{Question, QuestionType};

pub async fn judge_subjectives(questions: Vec<Question>, answer: HashMap<String, String>) -> i32 {
    let mut score = 0;
    let mut full_score = 0;
    
    for question in questions {
        if let Some(ans) = question.answer { 
            full_score += ans.all_points;
            
            match question.r#type {
                QuestionType::MultipleChoice => {
                    let point = judge_multiple_choice(&ans.answer, answer.get(&question.id.to_string()).unwrap());
                    match point {
                        PointType::All => {
                            score += ans.all_points;
                        }
                        PointType::Sub => {
                            if let Some(sub) = ans.sub_points {
                                score += sub;
                            }
                        }
                        PointType::None => {}
                    }
                }
                QuestionType::SingleChoice => {
                    let point = judge_single_choice(&ans.answer, answer.get(&question.id.to_string()).unwrap());
                    match point {
                        PointType::All => {
                            score += ans.all_points;
                        }
                        PointType::None => {}
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    score
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