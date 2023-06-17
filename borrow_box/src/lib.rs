#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        let game_session=GameSession{
            id:id,
            p1:(p1_name,0),
            p2:(p2_name,0),
            nb_games:nb_games
        };
        Box::new(game_session)
    }
    pub fn read_winner(&self) -> (String, u16) {
        let winner1 = &self.p1.0;// we can also declared a tuples let (p1_name, p1_score) = &self.p1;
        let score1 = &self.p1.1;                                  //let (p2_name, p2_score) = &self.p2; like this thnx. 
        let winner2 = &self.p2.0;
        let score2 = &self.p2.1;
        if score1 > score2{
            return (winner1.to_string(),*score1);
        }else if score2 > score1{
            return (winner2.to_string(),*score2);
        } else {
            (String::from("Same score! tied"),*score1)
        }
    }
    pub fn update_score(&mut self, user_name: String) {
            let nbr_games=0;
            let (p1_name, p1_score) = &mut self.p1;
            let (p2_name, p2_score) = &mut self.p2;
            if *p1_score == 3 || *p2_score == 3 || nbr_games==5{
                return;
            }
            if user_name == *p1_name{
                *p1_score += 1;
            } else if user_name == *p2_name {
                *p2_score += 1;
            } 
    }
    
    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}
    