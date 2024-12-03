import { useEffect, useState } from "preact/hooks";
import preactLogo from "../../assets/preact.svg";
import "./style.css";
import { Thread, ThreadSelector } from "../../components/ThreadSelctor";
import { Post } from "../../components/Post";
import { ThreadList } from "../../components/ThreadList";
import { ThreadView } from "../../components/ThreadView";
import { PostInputDialog } from "../../components/PostInputDialog";
import axios from "axios";
import { User, UserPanel } from "../../components/UserPanel";

export function Home() {
  const [selectedThread, setSelectedThread] = useState<Thread>(null);

  const [user, setUser] = useState<User>(null);

  const onSelectThread = (thread: Thread) => {
    setSelectedThread(thread);
    console.debug(selectedThread);
  };

  const handleUser = (user: User) => {
    setUser(user);
  };
  const onSubmit = (post: string) => {
    const data = {
      thread_id: selectedThread.id,
      author_id: user ? user.id : null,
      content: post,
    };

    axios
      .post("http://localhost:8080/api/posts", data, {
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
      })
      .then((response) => {
        console.log("Success:", response.data);
      })
      .catch((error) => {
        console.error("Error:", error);
      });
  };
  console.log("selected thread:", selectedThread);

  return (
    <div class="home">
      <div class="sidebar">
        <h2>Threads</h2>
        <ThreadList onSelectThread={onSelectThread} />
        <UserPanel user={user} handleUser={handleUser} />
      </div>
      {selectedThread ? (
        <div class="content">
          <ThreadView selectedThread={selectedThread} />
          <PostInputDialog onSubmit={onSubmit} />
        </div>
      ) : (
        <div class="content">
          <p>No thread selected.</p>
        </div>
      )}
    </div>
  );
}
