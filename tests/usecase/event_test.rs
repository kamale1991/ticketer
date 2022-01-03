#[macro_use]
extern crate domain_derive;

#[macro_use]
extern crate snafu;

use domain_patterns::collections::*;
pub mod common;
use common::*;
use uuid::Uuid;
use crate::common::UserEvents::UserCreated;
use domain_patterns::event::DomainEvent;

// Usecase はインターフェースだけを使って書く
// 抽象化したものをApplicationに追加する
// Entityの調整に徹するように書く

#[test]
#[allow(unused)]
fn test_signup_user_account() {

}

#[test]
#[allow(unused)]
fn test_modify_user_account() {

}

#[test]
#[allow(unused)]
fn test_signup_user_account() {

}

#[test]
#[allow(unused)]
fn test_delete_user_account() {

}

#[test]
#[allow(unused)]
fn test_signup_user_account() {

}

#[test]
#[allow(unused)]
fn test_store_event() {
    let user_id = Uuid::new_v4();
    let test_user = common::create_test_user(&user_id);
    let user_created_event = UserCreatedEvent::new(&test_user);
    let mut user_event_repo = UserEventRepository::new();

    user_event_repo.insert(&UserCreated(user_created_event.clone()));
    assert!(user_event_repo.contains_aggregate(&user_created_event.aggregate_id()));
    assert!(user_event_repo.contains_event(&user_created_event.id()));
}

#[test]
#[allow(unused)]
fn test_retrieve_event() {
    let user_id = Uuid::new_v4();
    let test_user = common::create_test_user(&user_id);
    let user_created_event = UserCreatedEvent::new(&test_user);
    let mut user_event_repo = UserEventRepository::new();

    user_event_repo.insert(&UserCreated(user_created_event.clone()));
    let event = user_event_repo.get(&user_created_event.id()).unwrap();

    let mut mutated_for_failure = user_created_event.clone();
    mutated_for_failure.id = Uuid::new_v4();

    let unpacked_event = match event {
        UserCreated(e) => e,
        _ => mutated_for_failure,
    };

    assert_eq!(unpacked_event.id, user_created_event.id);
}