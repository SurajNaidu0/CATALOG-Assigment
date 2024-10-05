use std::collections::HashMap;
use std::time::{Duration, Instant};
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
enum ServerState {
    Follower,
    Candidate,
    Leader,
}

struct Server {
    id: u64,
    state: ServerState,
    current_term: u64,
    voted_for: Option<u64>,
    log: Vec<String>,
    commit_index: usize,
    last_applied: usize,
    next_index: HashMap<u64, usize>,
    match_index: HashMap<u64, usize>,
    last_heartbeat: Instant,
    election_timeout: Duration,
}

impl Server {
    fn new(id: u64) -> Self {
        Server {
            id,
            state: ServerState::Follower,
            current_term: 0,
            voted_for: None,
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
            next_index: HashMap::new(),
            match_index: HashMap::new(),
            last_heartbeat: Instant::now(),
            election_timeout: Duration::from_millis(rand::thread_rng().gen_range(150..300)),
        }
    }

    fn start_election(&mut self) {
        self.state = ServerState::Candidate;
        self.current_term += 1;
        self.voted_for = Some(self.id);
        self.last_heartbeat = Instant::now();
        println!("Server {} started election for term {}", self.id, self.current_term);
    }

    fn become_leader(&mut self) {
        self.state = ServerState::Leader;
        println!("Server {} became leader for term {}", self.id, self.current_term);
    }

    fn append_entries(&mut self, term: u64, leader_id: u64, entries: Vec<String>) {
        if term < self.current_term {
            return;
        }

        if term > self.current_term {
            self.current_term = term;
            self.state = ServerState::Follower;
            self.voted_for = None;
        }

        self.last_heartbeat = Instant::now();
        println!("Server {} received entries from leader {}", self.id, leader_id);
    }

    fn request_vote(&mut self, term: u64, candidate_id: u64) -> bool {
        if term > self.current_term {
            self.current_term = term;
            self.state = ServerState::Follower;
            self.voted_for = None;
        }

        if term == self.current_term && (self.voted_for.is_none() || self.voted_for == Some(candidate_id)) {
            self.voted_for = Some(candidate_id);
            println!("Server {} voted for candidate {} in term {}", self.id, candidate_id, term);
            true
        } else {
            false
        }
    }

    fn check_timeout(&mut self) {
        if self.state != ServerState::Leader && self.last_heartbeat.elapsed() > self.election_timeout {
            self.start_election();
        }
    }
}

fn main() {
    let mut servers: Vec<Server> = (0..5).map(|i| Server::new(i)).collect();

    // Simulate some basic Raft behavior
    for _ in 0..10 {
        for server in servers.iter_mut() {
            server.check_timeout();
        }

        // Simulate leader sending heartbeats
        if let Some(leader) = servers.iter().find(|s| matches!(s.state, ServerState::Leader)) {
            let leader_id = leader.id;
            let leader_term = leader.current_term;
            for follower in servers.iter_mut().filter(|s| s.id != leader_id) {
                follower.append_entries(leader_term, leader_id, Vec::new());
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}