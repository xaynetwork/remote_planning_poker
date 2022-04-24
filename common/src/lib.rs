use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameAction {
    CurrentState(Game),
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
pub struct GameId(pub Uuid);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: GameId,
    pub stories: HashMap<StoryId, Story>,
    pub players: HashMap<UserId, Player>,
}

impl Game {
    pub fn new(user: User) -> Self {
        let id = GameId(Uuid::new_v4());
        let player = Player::new_admin(user);
        let mut players: HashMap<UserId, Player> = HashMap::new();
        players.insert(player.user.id, player);

        Game {
            id,
            players,
            stories: HashMap::new(),
        }
    }

    pub fn reduce(mut self, message: GameMessage) -> Self {
        match message {
            // only deal with messages concerning same game
            GameMessage { game_id, .. } if game_id == self.id => {
                // check if player has already joined the game
                match self.players.get(&message.user_id) {
                    Some(player) => {
                        let is_admin = player.role == PlayerRole::Admin;
                        match message.action {
                            GameAction::StoryAdded(story_info) if is_admin => {
                                self.add_story(story_info)
                            }
                            GameAction::StoryUpdated(story_id, story_info) if is_admin => {
                                self.update_story(&story_id, story_info)
                            }
                            GameAction::StoryRemoved(story_id) if is_admin => {
                                self.remove_story(&story_id)
                            }
                            GameAction::VotingOpened(story_id) if is_admin => {
                                self.open_story_for_voting(&story_id)
                            }
                            GameAction::VotesRevealed(story_id) if is_admin => {
                                self.reveal_votes(&story_id)
                            }
                            GameAction::VoteCasted(story_id, vote) => {
                                let player_id = player.user.id;
                                self.cast_vote(&story_id, player_id, vote)
                            }
                            GameAction::PlayerLeft => self.remove_player(&message.user_id),
                            _ => (),
                        }
                    }
                    None => match message.action {
                        // non-registered players can only join the game
                        GameAction::PlayerJoined(user) => self.add_player(user),
                        _ => (),
                    },
                }
            }
            _ => (),
        }
        self
    }

    fn add_player(&mut self, user: User) {
        let player = Player::new(user);
        println!("add player: {:#?}", &player);
        self.players.insert(player.user.id, player);
    }

    fn remove_player(&mut self, user_id: &UserId) {
        self.players.remove(user_id);
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
            Some(story) if story.status == StoryStatus::Init && any_open_for_voting == false => {
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
pub struct UserId(pub Uuid);

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
}

impl Player {
    pub fn new(user: User) -> Self {
        let role = PlayerRole::Player;
        Player { user, role }
    }

    pub fn new_admin(user: User) -> Self {
        let role = PlayerRole::Admin;
        Player { user, role }
    }
}
