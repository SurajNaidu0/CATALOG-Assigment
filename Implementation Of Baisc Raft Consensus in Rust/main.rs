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
    log: Vec<u64>,
    commit_index: u64,
    last_applied: u64,


}
fn main(){

}