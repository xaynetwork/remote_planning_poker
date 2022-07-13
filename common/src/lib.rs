use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameAction {
    PlayerJoined(User),
    PlayerLeft,
    StoriesAdded(Vec<BacklogStory>),
    StoryUpdated(StoryId, StoryInfo),
    StoryPositionChanged(StoryId, usize),
    StoryRemoved(StoryId),
    VotingOpened(StoryId),
    VotingClosed,
    VoteCasted(Vote),
    VotesRevealed,
    VotesCleared,
    ResultsApproved(Option<Vote>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AppEvent {
    CurrentState(Game),
    GameNotFound(GameId),
    GameMessage(UserId, GameAction),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, derive_more::Display)]
pub struct GameId(Uuid);

impl GameId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: GameId,
    players: IndexMap<UserId, Player>,
    backlog_stories: IndexMap<StoryId, BacklogStory>,
    estimated_stories: IndexMap<StoryId, EstimatedStory>,
    selected_story: Option<SelectedStory>,
}

impl Game {
    pub fn new(user: User) -> Self {
        let player = Player::new_admin(user);
        let players = [(player.user.id, player)].into_iter().collect();

        Game {
            id: GameId(Uuid::new_v4()),
            backlog_stories: IndexMap::new(),
            estimated_stories: IndexMap::new(),
            selected_story: None,
            players,
        }
    }

    pub fn to_active_players(&self) -> IndexMap<UserId, Player> {
        self.players
            .iter()
            .filter_map(|(user_id, player)| player.active.then_some((*user_id, player.clone())))
            .collect()
    }

    pub fn update(&mut self, user_id: UserId, action: GameAction) {
        if let GameAction::PlayerJoined(user) = action {
            self.add_player(user);
        } else if let Some(player) = self.players.get(&user_id) {
            let is_admin = player.role == PlayerRole::Admin;
            match action {
                GameAction::StoriesAdded(stories) if is_admin => self.add_stories(stories),
                GameAction::StoryUpdated(story_id, story_info) if is_admin => {
                    self.update_story(story_id, story_info)
                }
                GameAction::StoryPositionChanged(story_id, idx) => {
                    self.change_story_position(story_id, idx)
                }
                GameAction::StoryRemoved(story_id) if is_admin => self.remove_story(story_id),
                GameAction::VotingOpened(story_id) if is_admin => {
                    self.open_story_for_voting(story_id)
                }
                GameAction::VotingClosed if is_admin => self.close_story_for_voting(),
                GameAction::VotesRevealed if is_admin => self.reveal_votes(),
                GameAction::VotesCleared if is_admin => self.clear_votes(),
                GameAction::ResultsApproved(estimate) if is_admin => self.accept_round(estimate),
                GameAction::VoteCasted(vote) => self.cast_vote(user_id, vote),
                GameAction::PlayerLeft => self.remove_player(&user_id),
                // we don't process the rest
                GameAction::StoriesAdded(_)
                | GameAction::StoryUpdated(_, _)
                | GameAction::StoryRemoved(_)
                | GameAction::PlayerJoined(_)
                | GameAction::ResultsApproved(_)
                | GameAction::VotingOpened(_)
                | GameAction::VotingClosed
                | GameAction::VotesCleared
                | GameAction::VotesRevealed => (),
            };
        }
    }

    fn add_player(&mut self, user: User) {
        self.players
            .entry(user.id)
            .or_insert_with(|| Player::new(user, PlayerRole::Player))
            .active = true
    }

    fn remove_player(&mut self, user_id: &UserId) {
        if let Some(player) = self.players.get_mut(user_id) {
            player.active = false;
        }
    }

    fn add_stories(&mut self, stories: Vec<BacklogStory>) {
        self.backlog_stories
            .extend(stories.into_iter().map(|s| (s.id, s)));
    }

    fn change_story_position(&mut self, story_id: StoryId, new_idx: usize) {
        if let Some((idx, _, _)) = self.backlog_stories.get_full(&story_id) {
            self.backlog_stories.move_index(idx, new_idx);
        }
    }

    fn update_story(&mut self, story_id: StoryId, info: StoryInfo) {
        if let Some(story) = self.backlog_stories.get_mut(&story_id) {
            story.info = info;
        }
    }

    fn remove_story(&mut self, story_id: StoryId) {
        self.backlog_stories.shift_remove(&story_id);
    }

    fn open_story_for_voting(&mut self, story_id: StoryId) {
        // if there was a story already open for voting add it back to backlog
        self.close_story_for_voting();

        if let Some(story) = self.backlog_stories.shift_remove(&story_id) {
            self.selected_story = story.select_for_estimation().into();
        }
    }

    fn close_story_for_voting(&mut self) {
        if let Some(story) = self.selected_story.take() {
            let story = story.into_backlog();
            self.backlog_stories.insert(story.id, story);
        }
    }

    fn cast_vote(&mut self, player_id: UserId, vote: Vote) {
        if let Some(ref mut story) = self.selected_story {
            story.add_vote(player_id, vote);
        }
    }

    fn reveal_votes(&mut self) {
        match &mut self.selected_story {
            Some(story) if !story.votes_revealed && !story.votes.is_empty() => {
                story.reveal_votes();
                self.selected_story = story.clone().into();
            }
            _ => (),
        }
    }

    fn clear_votes(&mut self) {
        if let Some(story) = &mut self.selected_story {
            story.clear_votes();
            self.selected_story = story.clone().into();
        }
    }

    fn accept_round(&mut self, estimate: Option<Vote>) {
        match &self.selected_story {
            Some(story) if story.votes_revealed && !story.votes.is_empty() => {
                let estimate = estimate.unwrap_or_else(|| {
                    let avrg = story.votes_avrg();
                    Vote::get_closest_vote(&avrg)
                });
                let story = story.accept_with_estimate(estimate);
                self.selected_story = None;
                self.estimated_stories.insert(story.id, story);
            }
            _ => (),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, Debug, derive_more::Display)]
pub struct StoryId(Uuid);

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct StoryInfo {
    pub title: String,
}

/// Story that is waiting in the backlog to be selected for estimation.
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct BacklogStory {
    pub id: StoryId,
    pub info: StoryInfo,
}

impl BacklogStory {
    pub fn new(info: StoryInfo) -> Self {
        BacklogStory {
            id: StoryId(Uuid::new_v4()),
            info,
        }
    }

    pub fn select_for_estimation(&self) -> SelectedStory {
        SelectedStory {
            id: self.id,
            info: self.info.clone(),
            votes: IndexMap::new(),
            votes_revealed: false,
        }
    }
}

/// Story that is selected for estimation.
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct SelectedStory {
    pub id: StoryId,
    pub info: StoryInfo,
    pub votes: IndexMap<UserId, Vote>,
    pub votes_revealed: bool,
}

impl SelectedStory {
    pub fn can_accept(&self) -> bool {
        self.votes_revealed && !self.votes.is_empty()
    }

    pub fn can_play_again(&self) -> bool {
        !self.votes.is_empty()
    }

    pub fn can_reveal(&self) -> bool {
        !self.votes_revealed && !self.votes.is_empty()
    }

    pub fn add_vote(&mut self, player_id: UserId, vote: Vote) {
        self.votes.insert(player_id, vote);
    }

    pub fn reveal_votes(&mut self) {
        self.votes_revealed = true;
    }

    pub fn clear_votes(&mut self) {
        self.votes_revealed = false;
        self.votes.clear();
    }

    pub fn accept_with_estimate(&self, estimate: Vote) -> EstimatedStory {
        EstimatedStory {
            id: self.id,
            info: self.info.clone(),
            estimate,
        }
    }

    pub fn into_backlog(self) -> BacklogStory {
        BacklogStory {
            id: self.id,
            info: self.info,
        }
    }

    pub fn votes_avrg(&self) -> f32 {
        if self.votes.is_empty() {
            0.
        } else {
            let val = self.votes.iter().map(|(_, vote)| vote.0).sum::<i32>();
            val as f32 / self.votes.len() as f32
        }
    }
}

/// Story that is estimated.
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct EstimatedStory {
    pub id: StoryId,
    pub info: StoryInfo,
    pub estimate: Vote,
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, Debug, derive_more::Display)]
pub struct UserId(Uuid);

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: UserId,
    // TODO: move this maybe to Player struct to allow for different names in different games/teams
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
