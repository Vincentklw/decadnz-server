use chrono::NaiveDateTime;
use uuid::Uuid;

struct Policy {
    pub policy_id: Uuid,
    pub policy_name: String,
    pub policy_rule: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}