use std::collections::HashMap;
use std::time::{Duration, Instant};
use rand::Rng;

enum ServerState{
    Leader,
    Candidate,
    Follower,
}

struct Server {
    id: u64,
    state: ServerState,
    current_term: u64,
    voted_for: Option<u64>,
    vote_count:u64,
    node:Vec<Server>,
    candidate_map :HashMap<u64, Server>,
    log: Vec<u64>,
    commit_index: HashMap<Server, u64>,
    last_applied: HashMap<Server, u64>,
    last_heartbeat:Instant,
    election_timeout:Duration,
}
impl Server {
    // Insisting new node
    fn new(id: u64) -> Self{
        let server = Server{ id : id,
            state: ServerState::Follower,
            current_term:0,
            voted_for: None,
            vote_count:0,
            node:Vec::new(),
            candidate_map:HashMap::new(),
            log: Vec::new(),
            commit_index: HashMap::new(),
            last_applied: HashMap::new(),
            last_heartbeat: Instant::now(),
            election_timeout: Duration::from_millis(rand::thread_rng().gen_range(150..300)), };
        return server;
    }
    //Starting a new election
    fn start_election(&mut self) {
        self.state = ServerState::Candidate;
        self.current_term += 1;
        self.voted_for = Some(self.current_term);
        self.vote_count = 1;
        self.last_heartbeat = Instant::now();
        // Request votes from other nodes
        for node in self.node.iter_mut() {
            node.request_vote(self.current_term,self.id);
        }

        for node in self.node.iter() {
            if node.id != self.id{
                match node.voted_for {
                    None => {println!("No vote found for {}",node.id)}
                    Some(t) => {
                        if t== self.id{
                            self.vote_count+=1
                        }
                    }
                }

            }
        }
        if self.vote_count > self.node.len() as u64/2 {
            self.become_leader();
        }
    }
    //Become Leader
    fn become_leader(&mut self) {
        self.state = ServerState::Leader;
        println!("Node {} became the leader for term {}", self.id, self.current_term);
        self.send_heartbeats();
    }

    // Requesting vote
    fn request_vote(&mut self, term:u64, candidate_id:u64) {
        if term > self.current_term {
            self.voted_for = Some(candidate_id);
            let node =self.candidate_map.get(&candidate_id);
            match node{
                Option::Some(_node) => {self.voted_for = Some(candidate_id);},
                None=>println!("No node found Wrong Message")
            }
        }
    }
    // Send Heartbeat
    fn send_heartbeats(&mut self) {
        for node in self.node.iter_mut() {
            node.last_heartbeat = Instant::now();
        }
    }

}
fn main() {
    println!("Raft Implemented using rust");
    println!("If it compiles in rust trust it will run ");

}