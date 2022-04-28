use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameAction {
    CurrentState(Game),
    GameNotFound(GameId),
    PlayerJoined(User),
    PlayerLeft,
    StoryAdded(StoryInfo),
    StoryUpdated(StoryId, StoryInfo),
    StoryRemoved(StoryId),
    VotingOpened(StoryId), // also for reopening voting
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

    pub fn reduce(mut self, message: GameMessage) -> Self {
        if self.id != message.game_id {
            return self;
        }

        if let GameAction::PlayerJoined(user) = message.action.clone() {
            self.add_player(user);
        } else if let Some(player) = self.players.get(&message.user_id) {
            let is_admin = player.role == PlayerRole::Admin;
            match message.action {
                GameAction::StoryAdded(story_info) if is_admin => self.add_story(story_info),
                GameAction::StoryUpdated(story_id, story_info) if is_admin => {
                    self.update_story(&story_id, story_info)
                }
                GameAction::StoryRemoved(story_id) if is_admin => self.remove_story(&story_id),
                GameAction::VotingOpened(story_id) if is_admin => {
                    self.open_story_for_voting(&story_id)
                }
                GameAction::VotesRevealed(story_id) if is_admin => self.reveal_votes(&story_id),
                GameAction::VoteCasted(story_id, vote) => {
                    let player_id = player.user.id;
                    self.cast_vote(&story_id, player_id, vote)
                }
                GameAction::PlayerLeft => self.remove_player(&message.user_id),
                // we don't process the rest
                GameAction::StoryAdded(_)
                | GameAction::StoryUpdated(_, _)
                | GameAction::StoryRemoved(_)
                | GameAction::CurrentState(_)
                | GameAction::GameNotFound(_)
                | GameAction::PlayerJoined(_)
                | GameAction::ResultsApproved(_)
                | GameAction::VotingOpened(_)
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
        println!("add_player: {:#?},", player);
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

    fn add_story(&mut self, info: StoryInfo) {
        let story = Story::new(info);
        self.stories.insert(story.id, story);
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
        let any_open_for_voting = self
            .stories
            .values()
            .any(|s| s.status == StoryStatus::Voting);

        match self.stories.get(story_id) {
            Some(story) if story.status == StoryStatus::Init && !any_open_for_voting => {
                let mut story = story.clone();
                story.status = StoryStatus::Voting;
                self.stories.insert(story.id, story);
            }
            _ => (),
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
            Some(story) if story.status == StoryStatus::Voting => {
                let mut story = story.clone();
                story.status = StoryStatus::Revealed;
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

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum StoryStatus {
    Init,
    Voting,
    Revealed,
    Approved,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StoryInfo {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
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
    Break = -1,
    QuestionMark = -2,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
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
