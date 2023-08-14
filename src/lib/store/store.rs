use std::time::Duration;
use moka::sync::Cache;
use super::models::{Challenge, Session, User};


pub struct DataStore {
    users: Cache<String, User>,
    sessions: Cache<String, Session>,
    challenges: Cache<String, Challenge>,
}

impl DataStore {
    pub fn new() -> Self {
        Self {
            users: Cache::builder().build(),
            challenges: Cache::builder().time_to_live(Duration::from_secs(10)).build(),
            sessions: Cache::builder().time_to_live(Duration::from_secs(360)).build(),
        }
    }

    pub fn insert_user(&self, user: User) {
        self.users.insert(user.id.clone(), user)
    }

    pub fn get_user(&self, id: &String) -> Option<User>{
        return self.users.get(id)
    }

    pub fn insert_challenge(&self, challenge: Challenge) {
        self.challenges.insert(challenge.id.clone(), challenge)
    }

    pub fn get_challenge(&self, id: &String) -> Option<Challenge>{
        return self.challenges.get(id)
    }

    pub fn insert_session(&self, session: Session) {
        self.sessions.insert(session.id.clone(), session)
    }

    pub fn get_session(&self, id: &String) -> Option<Session>{
        return self.sessions.get(id)
    }
} 