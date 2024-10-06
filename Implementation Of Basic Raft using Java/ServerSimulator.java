public class ServerSimulator{
    public static void main(String[] args) throws InterruptedException {
        System.out.println("Raft Simulator Started");
        HashMap<Integer, Server> servers = new HashMap<>();
        int nodeCount = 5;

        // Create servers
        for (int i = 1; i <= nodeCount; i++) {
            servers.put(i, new Server(i));
        }

        // Add peers to each server
        for (Server server : servers.values()) {
            for (Server node : servers.values()) {
                if (server.id != node.id) {
                    server.addServer(node);
                }
            }