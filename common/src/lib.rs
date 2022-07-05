use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameAction {
    PlayerJoined(User),
    PlayerLeft,
    StoriesAdded(Vec<Story>),
    StoryUpdated(StoryId, StoryInfo),
    StoryRemoved(StoryId),
    VotingOpened(StoryId),
    VotingClosed(StoryId),
    VoteCasted(StoryId, Vote),
    VotesRevealed(StoryId),
    ResultsApproved(StoryId, Option<Vote>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AppMessage {
    CurrentState(Game),
    GameNotFound(GameId),
    GameMessage(UserId, GameAction),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct GameId(Uuid);

impl GameId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for GameId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: GameId,
    pub stories: IndexMap<StoryId, Story>,
    pub players: IndexMap<UserId, Player>,
}

impl Game {
    pub fn new(user: User) -> Self {
        let player = Player::new_admin(user);
        let mut players = IndexMap::new();
        players.insert(player.user.id, player);

        Game {
            id: GameId(Uuid::new_v4()),
            stories: IndexMap::new(),
            players,
        }
    }

    pub fn stories_by_filter(&self, filter: fn(&Story) -> bool) -> Vec<Story> {
        self.stories
            .iter()
            .filter(|(_, story)| filter(story))
            .map(|(_, story)| story.clone())
            .collect()
    }

    pub fn active_players(&self) -> Vec<Player> {
        self.players
            .iter()
            .filter(|(_, player)| player.active)
            .map(|(_, player)| player.clone())
            .collect()
    }

    pub fn reduce(mut self, user_id: UserId, action: GameAction) -> Self {
        if let GameAction::PlayerJoined(user) = action.clone() {
            self.add_player(user);
        } else if let Some(player) = self.players.get(&user_id) {
            let is_admin = player.role == PlayerRole::Admin;
            match action {
                GameAction::StoriesAdded(stories) if is_admin => self.add_stories(stories),
                GameAction::StoryUpdated(story_id, story_info) if is_admin => {
                    self.update_story(&story_id, story_info)
                }
                GameAction::StoryRemoved(story_id) if is_admin => self.remove_story(&story_id),
                GameAction::VotingOpened(story_id) if is_admin => {
                    self.open_story_for_voting(&story_id)
                }
                GameAction::VotingClosed(story_id) if is_admin => {
                    self.close_story_for_voting(&story_id)
                }
                GameAction::VotesRevealed(story_id) if is_admin => self.reveal_votes(&story_id),
                GameAction::ResultsApproved(story_id, estimate) if is_admin => {
                    self.accept_round(&story_id, estimate)
                }
                GameAction::VoteCasted(story_id, vote) => {
                    let player_id = player.user.id;
                    self.cast_vote(&story_id, player_id, vote)
                }
                GameAction::PlayerLeft => self.remove_player(&user_id),
                // we don't process the rest
                GameAction::StoriesAdded(_)
                | GameAction::StoryUpdated(_, _)
                | GameAction::StoryRemoved(_)
                | GameAction::PlayerJoined(_)
                | GameAction::ResultsApproved(_, _)
                | GameAction::VotingOpened(_)
                | GameAction::VotingClosed(_)
                | GameAction::VotesRevealed(_) => (),
            };
        }
        self
    }

    fn add_player(&mut self, user: User) {
        let player = match self.players.get(&user.id) {
            Some(player) => Player {
                active: true,
                ..(*player).clone()
            },
            None => Player::new(user, PlayerRole::Player),
        };
        self.players.insert(player.user.id, player);
    }

    fn remove_player(&mut self, user_id: &UserId) {
        if let Some(player) = self.players.get(user_id) {
            let player = Player {
                active: false,
                ..(*player).clone()
            };
            self.players.insert(player.user.id, player);
        } else {
            // user didn't registered in the game
        }
    }

    fn add_stories(&mut self, stories: Vec<Story>) {
        let to_add: IndexMap<_, _> = stories.into_iter().map(|s| (s.id, s)).collect();
        self.stories.extend(to_add);
    }

    fn update_story(&mut self, story_id: &StoryId, info: StoryInfo) {
        if let Some(story) = self.stories.get(story_id) {
            let story = Story {
                info,
                ..story.clone()
            };
            self.stories.insert(story.id, story);
        }
    }

    fn remove_story(&mut self, story_id: &StoryId) {
        self.stories.remove(story_id);
    }

    fn open_story_for_voting(&mut self, story_id: &StoryId) {
        let stories: IndexMap<StoryId, Story> = self
            .stories
            .clone()
            .into_iter()
            .map(|(id, story)| {
                let story = match story.status {
                    // do nothing for approved stories
                    StoryStatus::Approved => story,
                    // open story for voting or reopen story (clear votes)
                    StoryStatus::Init | StoryStatus::Voting | StoryStatus::Revealed
                        if story_id == &id =>
                    {
                        Story {
                            status: StoryStatus::Voting,
                            votes: IndexMap::new(),
                            ..story
                        }
                    }
                    // close other stories for voting
                    _ => Story {
                        status: StoryStatus::Init,
                        votes: IndexMap::new(),
                        ..story
                    },
                };
                (id, story)
            })
            .collect();

        self.stories = stories;
    }

    fn close_story_for_voting(&mut self, story_id: &StoryId) {
        if let Some(story) = self.stories.get(story_id) {
            let story = Story {
                status: StoryStatus::Init,
                votes: IndexMap::new(),
                ..story.clone()
            };
            self.stories.insert(story.id, story);
        }
    }

    fn cast_vote(&mut self, story_id: &StoryId, player_id: UserId, vote: Vote) {
        match self.stories.get(story_id) {
            Some(story) if story.status == StoryStatus::Voting => {
                let mut story = story.clone();

                story.votes.insert(player_id, vote);
                self.stories.insert(story.id, story);
            }
            _ => (),
        }
    }

    fn reveal_votes(&mut self, story_id: &StoryId) {
        match self.stories.get(story_id) {
            Some(story) if story.status == StoryStatus::Voting && !story.votes.is_empty() => {
                let mut story = story.clone();
                story.status = StoryStatus::Revealed;
                self.stories.insert(story.id, story);
            }
            _ => (),
        }
    }

    fn accept_round(&mut self, story_id: &StoryId, estimate: Option<Vote>) {
        match self.stories.get(story_id) {
            Some(story) if story.status == StoryStatus::Revealed => {
                let estimate = estimate.unwrap_or_else(|| {
                    let avrg = story.votes_avrg();
                    Vote::get_closest_vote(&avrg)
                });
                let mut story = story.clone();
                story.status = StoryStatus::Approved;
                story.estimate = Some(estimate);
                self.stories.insert(story.id, story);
            }
            _ => (),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct StoryId(Uuid);

impl fmt::Display for StoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub enum StoryStatus {
    Init,
    Voting,
    Revealed,
    Approved,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct StoryInfo {
    pub title: String,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct Story {
    pub id: StoryId,
    pub info: StoryInfo,
    pub estimate: Option<Vote>,
    pub votes: IndexMap<UserId, Vote>,
    pub status: StoryStatus,
}

impl Story {
    pub fn new(info: StoryInfo) -> Self {
        Story {
            id: StoryId(Uuid::new_v4()),
            votes: IndexMap::new(),
            status: StoryStatus::Init,
            estimate: None,
            info,
        }
    }

    pub fn votes_avrg(&self) -> f32 {
        if self.votes.is_empty() {
            0f32
        } else {
            let val: i32 = self.votes.iter().map(|(_, vote)| vote.0).sum();
            val as f32 / self.votes.len() as f32
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Vote(i32);

const VOTES: [i32; 10] = [0, 1, 2, 3, 5, 8, 13, 21, 40, 100];

impl Vote {
    pub fn new(value: i32) -> Result<Vote, String> {
        if VOTES.contains(&value) {
            Ok(Self(value))
        } else {
            Err("Not allowed value".to_string())
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn get_allowed_values() -> [i32; 10] {
        VOTES
    }

    pub fn get_allowed_votes() -> [Vote; 10] {
        VOTES.map(|val| Vote::new(val).unwrap())
    }

    pub fn get_closest_vote(value: &f32) -> Vote {
        let closest = VOTES
            .iter()
            .reduce(|prev, curr| {
                if (*curr as f32 - value).abs() < (*prev as f32 - value).abs() {
                    curr
                } else {
                    prev
                }
            })
            .unwrap();
        Vote(*closest)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct UserId(Uuid);

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: UserId,
    pub name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        User {
            id: UserId(Uuid::new_v4()),
            name,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum PlayerRole {
    Admin,
    Player,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub user: User,
    pub role: PlayerRole,
    pub active: bool,
}

impl Player {
    pub fn new(user: User, role: PlayerRole) -> Self {
        let active = true;
        Player { user, role, active }
    }

    pub fn new_admin(user: User) -> Self {
        Player::new(user, PlayerRole::Admin)
    }
}
