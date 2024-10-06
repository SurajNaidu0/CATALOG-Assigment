use std::collections::HashMap;
use std::time::{Duration, Instant};
use rand::Rng;

#[derive(PartialEq, Clone, Copy)]
enum ServerState {
    LEADER,
    CANDIDATE,
    FOLLOWER,
}

struct Server {
    id: i32,
    state: ServerState,
    current_term: i32,
    voted_for: Option<i32>,
    vote_count: i32,
    nodes: HashMap<i32, Box<Server>>,
    last_heartbeat: Instant,
    election_timeout: Duration,
}

impl Server {
    fn new(id: i32) -> Self {
        Server {
            id,
            state: ServerState::FOLLOWER,
            current_term: 0,
            voted_for: None,
            vote_count: 0,
            nodes: HashMap::new(),
            last_heartbeat: Instant::now(),
            election_timeout: Duration::from_millis(rand::thread_rng().gen_range(150..300)),
        }
    }

    fn add_server(&mut self, server: Box<Server>) {
        if !self.nodes.contains_key(&server.id) {
            self.nodes.insert(server.id, server);
        }
    }

    fn start_election(&mut self) {
        self.state = ServerState::CANDIDATE;
        self.current_term += 1;
        self.voted_for = Some(self.id);
        self.vote_count = 1;
        self.last_heartbeat = Instant::now();
        println!("Node {} started election for term {}", self.id, self.current_term);

        for node in self.nodes.values_mut() {
            if node.request_vote(self.current_term, self.id) {
                self.vote_count += 1;
            }
        }

        if self.vote_count > (self.nodes.len() as i32 / 2) {
            self.become_leader();
        } else {
            println!("Node {} failed to get majority. Returning to follower state.", self.id);
            self.state = ServerState::FOLLOWER;
        }
    }

    fn become_leader(&mut self) {
        self.state = ServerState::LEADER;
        println!("Node {} became the leader for term {}", self.id, self.current_term);
        self.send_heartbeats();
    }

    fn send_heartbeats(&mut self) {
        for node in self.nodes.values_mut() {
            node.receive_heartbeat(self.current_term, self.id);
        }
    }

    fn receive_heartbeat(&mut self, term: i32, leader_id: i32) {
        if term >= self.current_term {
            self.current_term = term;
            self.state = ServerState::FOLLOWER;
            self.last_heartbeat = Instant::now();
            println!("Node {} received heartbeat from leader {}", self.id, leader_id);
        }
    }

    fn request_vote(&mut self, term: i32, candidate_id: i32) -> bool {
        if term > self.current_term {
            self.current_term = term;
            self.state = ServerState::FOLLOWER;
            self.voted_for = None;
        }
        if term == self.current_term && (self.voted_for.is_none() || self.voted_for == Some(candidate_id)) {
            self.voted_for = Some(candidate_id);
            return true;
        }
        false
    }

    fn check_timeout(&mut self) {
        if self.state != ServerState::LEADER &&
            Instant::now().duration_since(self.last_heartbeat) > self.election_timeout {
            self.start_election();
        }
    }
}

fn main() {
    println!("Raft algorithm implementation in Rust");
}