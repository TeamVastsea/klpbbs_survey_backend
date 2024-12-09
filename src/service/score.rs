use crate::dao::entity::question::QuestionType;
use crate::dao::entity::{page, score};
use crate::dao::model::question::Question;
use crate::DATABASE;
use log::info;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde_json::Value;
use std::collections::HashMap;

pub fn combine_answer(before: Value, change: Value) -> Value {
    let mut before = before.as_object().unwrap().clone();
    let change = change.as_object().unwrap();
    for (key, value) in change {
        before.insert(key.clone(), value.clone());
    }
    Value::Object(before)
}

impl score::Model {
    pub async fn judge_answer(self) -> Self {
        let answer: Value = serde_json::from_str(&self.answer).unwrap();
        let mut scores = HashMap::new();
        let (mut all_score, mut user_score) = (0, 0);


        let mut index = 0;
        let mut page = page::Model::get_by_survey_and_index(self.survey, index).await.unwrap();

        while index < page.1 {
            let questions = Question::find_by_page(page.0.id).await.unwrap();

            for question in questions {
                let Some(correct_answer) = question.answer else {
                    continue;
                };
                let all = correct_answer.all_points.unwrap_or(0);
                let sub = correct_answer.sub_points.unwrap_or(0);
                let Some(user_answer) = answer.get(question.id.to_string()) else {
                    scores.insert(question.id, 0);
                    all_score += all;
                    continue;
                };
                let user_answer = user_answer.as_str().unwrap();
                
                if user_answer.is_empty() || correct_answer.answer.is_empty() {
                    continue;
                }

                let score = match question.r#type {
                    QuestionType::Text | QuestionType::SingleChoice => {
                        if user_answer == correct_answer.answer {
                            all
                        } else {
                            0
                        }
                    }
                    QuestionType::MultipleChoice => {
                        let user_answer: Vec<String> = serde_json::from_str(user_answer).unwrap();
                        let correct_answer: Vec<String> = serde_json::from_str(&correct_answer.answer).unwrap();

                        let mut flag_wrong = false;

                        for a in &user_answer {
                            if !correct_answer.contains(&a.to_string()) {
                                flag_wrong = true;
                                break;
                            }
                        }

                        if flag_wrong {
                            0
                        } else if user_answer.len() == correct_answer.len() {
                            all
                        } else {
                            sub
                        }
                    }
                };

                scores.insert(question.id, score);
                all_score += all;
                user_score += score;
            }

            index += 1;
            if index < page.1 {
                page = page::Model::get_by_survey_and_index(self.survey, index).await.unwrap();
            }
        }

        let mut score_active = self.into_active_model();
        score_active.full_scores = Set(Some(all_score));
        score_active.user_scores = Set(Some(user_score));
        score_active.scores = Set(Some(serde_json::to_string(&scores).unwrap()));
        score_active.judge_time = Set(Some(chrono::Utc::now().naive_local()));
        score_active.update(&*DATABASE).await.unwrap()
    }
}