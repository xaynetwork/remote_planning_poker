use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameAction {
    CurrentState(Game),
    GameNotFound(GameId),
    PlayerJoined(User),
    PlayerLeft,
    StoriesAdded(Vec<Story>),
    StoryUpdated(StoryId, StoryInfo),
    StoryRemoved(StoryId),
    VotingOpened(StoryId),
    VotingClosed(StoryId),
    VoteCasted(StoryId, VoteValue),
    VotesRevealed(StoryId),
    ResultsApproved(StoryId),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameMessage {
    pub user_id: UserId,
    pub game_id: GameId,
    pub action: GameAction,
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
    pub stories: HashMap<StoryId, Story>,
    pub players: HashMap<UserId, Player>,
}

impl Game {
    pub fn new(user: User) -> Self {
        let player = Player::new_admin(user);
        let mut players: HashMap<UserId, Player> = HashMap::new();
        players.insert(player.user.id, player);

        Game {
            id: GameId(Uuid::new_v4()),
            stories: HashMap::new(),
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

    pub fn reduce(mut self, message: GameMessage) -> Self {
        if self.id != message.game_id {
            return self;
        }

        if let GameAction::PlayerJoined(user) = message.action.clone() {
            self.add_player(user);
        } else if let Some(player) = self.players.get(&message.user_id) {
            let is_admin = player.role == PlayerRole::Admin;
            match message.action {
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
                GameAction::ResultsApproved(story_id) if is_admin => self.accept_round(&story_id),
                GameAction::VoteCasted(story_id, vote) => {
                    let player_id = player.user.id;
                    self.cast_vote(&story_id, player_id, vote)
                }
                GameAction::PlayerLeft => self.remove_player(&message.user_id),
                // we don't process the rest
                GameAction::StoriesAdded(_)
                | GameAction::StoryUpdated(_, _)
                | GameAction::StoryRemoved(_)
                | GameAction::CurrentState(_)
                | GameAction::GameNotFound(_)
                | GameAction::PlayerJoined(_)
                | GameAction::ResultsApproved(_)
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
        let to_add: HashMap<_, _> = stories.into_iter().map(|s| (s.id, s)).collect();
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
        let stories: HashMap<StoryId, Story> = self
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
                            votes: HashMap::new(),
                            ..story
                        }
                    }
                    // close other stories for voting
                    _ => Story {
                        status: StoryStatus::Init,
                        votes: HashMap::new(),
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
                votes: HashMap::new(),
                ..story.clone()
            };
            self.stories.insert(story.id, story);
        }
    }

    fn cast_vote(&mut self, story_id: &StoryId, player_id: UserId, value: VoteValue) {
        match self.stories.get(story_id) {
            Some(story) if story.status == StoryStatus::Voting => {
                let mut story = story.clone();
                let vote = Vote { value };

                story.votes.insert(player_id, vote);
                self.stories.insert(story.id, story);
            }
            _ => (),
        }
    }

    fn reveal_votes(&mut self, story_id: &StoryId) {
        // can be only when all non-admin players voted
        match self.stories.get(story_id) {
            Some(story) if story.status == StoryStatus::Voting && !story.votes.is_empty() => {
                let mut story = story.clone();
                story.status = StoryStatus::Revealed;
                self.stories.insert(story.id, story);
            }
            _ => (),
        }
    }

    fn accept_round(&mut self, story_id: &StoryId) {
        match self.stories.get(story_id) {
            Some(story) if story.status == StoryStatus::Revealed => {
                let mut story = story.clone();
                story.status = StoryStatus::Approved;
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
    pub votes: HashMap<UserId, Vote>,
    pub status: StoryStatus,
}

impl Story {
    pub fn new(info: StoryInfo) -> Self {
        Story {
            id: StoryId(Uuid::new_v4()),
            votes: HashMap::new(),
            status: StoryStatus::Init,
            info,
        }
    }

    pub fn estimation(&self) -> f32 {
        if self.votes.is_empty() {
            0f32
        } else {
            let val: u8 = self.votes.iter().map(|(_, vote)| vote.value as u8).sum();
            (f32::from(val) / self.votes.len() as f32).round()
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum VoteValue {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Five = 5,
    Eight = 8,
    Thirteen = 13,
    TwentyOne = 21,
    Fourty = 40,
    OneHundred = 100,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Vote {
    pub value: VoteValue,
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
