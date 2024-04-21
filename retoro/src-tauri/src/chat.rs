// Define a struct to represent the chat system
#[derive(Debug)]
struct ChatSystem {
    channels: HashMap<String, Network>,
    users: HashMap<String, User>,
}

// Implement methods for the ChatSystem struct
impl ChatSystem {
    fn new() -> Self {
        ChatSystem {
            channels: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn create_channel(&mut self, name: String) {
        self.channels.insert(
            name.clone(),
            Channel {
                name,
                participants: Vec::new(),
                messages: Vec::new(),
            },
        );
    }

    fn join_channel(&mut self, channel_name: String, username: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            channel.participants.push(username.clone());
        }
    }

    fn send_message(&mut self, channel_name: String, sender: String, content: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            let message = ChatMessage { sender, content };
            channel.messages.push(message);
        }
    }
}
