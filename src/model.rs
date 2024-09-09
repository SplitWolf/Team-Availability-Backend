use rrule::RRuleSet;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::FromRow;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Identifier {
//     pub id: i32
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(FromRow)]
pub struct IdentifiableTeam {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(FromRow)]
pub struct IdentifiableUser {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub user_id: i32
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(FromRow)]
pub struct IdentifiablePlayer {
    pub id: i32,
    pub user_id: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(FromRow)]
//TODO: Figure out if I'm using this
pub struct PlayerToTeam {
    pub player_id: i32,
    pub team_id: i32
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(FromRow)]
pub struct AvailableBlock {
    #[serde(
        deserialize_with = "deserialize_naive_time", //TODO: Update Chrono types is changed
        serialize_with = "serialize_naive_time"
    )]
    pub start_time: chrono::NaiveTime,
    #[serde(
        deserialize_with = "deserialize_naive_time", //TODO: Update Chrono types is changed
        serialize_with = "serialize_naive_time"
    )]
    pub end_time: chrono::NaiveTime,
    pub need_warning: bool,
    #[serde(
        deserialize_with = "deserialize_rrule_set",
        serialize_with = "serialize_rrule_set"
    )]
    #[sqlx(try_from = "String")]
    pub repeats: MyRRuleSet, // TODO: Decide if optional, if so add default derive
    pub player_id: i32
}

// Wrapper Type for RRulset To implment From<String>
#[derive(Debug, Clone)]
pub struct MyRRuleSet(RRuleSet);

impl From<String> for MyRRuleSet {
    fn from(value: String) -> Self {
        // TODO: Not use unwrap.
        MyRRuleSet(value.parse().unwrap())
    }
}

impl std::ops::Deref for MyRRuleSet {
    type Target = RRuleSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for MyRRuleSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

//TODO: Remove Debug, Clone
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(FromRow)]
pub struct IdentifiableAvailableBlock {
    pub id: i32,
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub inner_block: AvailableBlock
}



fn serialize_rrule_set<S>(x: &RRuleSet, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    serde::Serialize::serialize(&x.to_string(), serializer)
}

fn deserialize_rrule_set<'de, D>(deserializer: D) -> Result<MyRRuleSet, D::Error>
where
D: Deserializer<'de>,
{
    use serde::de::Error; 
    let rrule_string = String::deserialize(deserializer)?;
    rrule_string.parse::<RRuleSet>().map(|rruleset| {
        MyRRuleSet(rruleset)
    }).map_err(Error::custom)
}


fn serialize_naive_time<S>(x: &chrono::NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    serde::Serialize::serialize(&x.to_string(), serializer)
}

fn deserialize_naive_time<'de, D>(deserializer: D) -> Result<chrono::NaiveTime, D::Error>
where
D: Deserializer<'de>,
{
    use serde::de::Error; 
    let time_string = String::deserialize(deserializer)?;
    time_string.parse().map_err(Error::custom)
}