import java.time.Duration;
import java.time.Instant;
import java.util.HashMap;
import java.util.Random;

enum ServerState{
    LEADER,
    CANDIDATE,
    FOLLOWER
}
class Server{
    public int id;
    public ServerState state;
    public int currentTerm;
    public int votedFor;
    public int voteCount;
    public HashMap<Integer,Server> nodes;
    public Instant lastHeartbeat;
    public Duration electionTimeout;

    public Server(int id){
        this.id = id;
        this.state = ServerState.FOLLOWER;
        this.currentTerm = 0;
        this.votedFor = Integer.parseInt(null);
        this.voteCount = 0;
        this.nodes = new HashMap<>();
        this.lastHeartbeat = Instant.now();
        this.electionTimeout = Duration.ofMillis(new Random().nextInt(150,300));
    }

    public void addServer(Server server){
      if (!this.nodes.containsKey(server.id)){
          this.nodes.put(server.id,server);
      }
    }
    public void startElection(){
       this.state = ServerState.CANDIDATE;
       this.currentTerm++;
       this.votedFor = this.id;
       this.voteCount = 1;
       this.lastHeartbeat = Instant.now();
       System.out.println("Node " + this.id + " started election for term" + this.currentTerm);

       for (Server node : this.nodes.values()){
           if (node.requestVote(this.currentTerm,this.id)){
               this.voteCount++;
           }
       }
       if (this.voteCount > this.nodes.size()/2){
           this.becomeLeader();
       }else{
           System.out.println("Node " + this.id + " failed to get majority. Returning to follower state.");
           this.state = ServerState.FOLLOWER;
       }
    }

    private void becomeLeader() {
        this.state = ServerState.LEADER;
        System.out.println("Node " + this.id + " became the leader for term" + this.currentTerm);
        this.sendHeartbeats();
    }

    private void sendHeartbeats() {
        for (Server node : this.nodes.values()){
            node.receiveHeartbeat(this.currentTerm,this.id);
        }
    }

    private void receiveHeartbeat(int term, int leaderId) {
        if (term >= this.currentTerm){
            this.currentTerm = term;
            this.state = ServerState.FOLLOWER;
            this.lastHeartbeat = Instant.now();
            System.out.println("Node "+ this.id + " received heartbeat from leader " + leaderId);

        }
    }

    public boolean requestVote(int term, int candidateId) {
        if (term > this.currentTerm){
            this.currentTerm = term;
            this.state = ServerState.FOLLOWER;
            this.votedFor = Integer.parseInt(null);
        }
        if ((term == this.currentTerm) && ((this.votedFor == Integer.parseInt(null)) || (this.votedFor == candidateId))){
            this.votedFor = candidateId;
            return true;
        }
        return false;
    }

    public void checkTimeout(){
        if (this.state != ServerState.LEADER &&
                Duration.between(this.lastHeartbeat, Instant.now()).compareTo(this.electionTimeout) > 0) {
            this.startElection();
        }

        }
    }
    
