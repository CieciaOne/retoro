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
  const [refreshKey, setRefreshKey] = useState(0);

  const checkSession = async () => {
    const cookies = document.cookie.split("; ");
    const sessionCookie = cookies.find((cookie) =>
      cookie.startsWith("session_id=")
    );

    if (sessionCookie) {
      const session_id = sessionCookie.split("=")[1];
      console.log(cookies, sessionCookie);

      try {
        const response = await axios.post(
          "http://localhost:8080/api/users/auth",
          { session_id: session_id },
          {
            headers: {
              Accept: "application/json",
              "Content-Type": "application/json",
            },
          }
        );

        if (response.status === 200) {
          setUser(response.data); // Set the user with data from the API
          console.log("User authenticated:", response.data);
        }
      } catch (error) {
        if (error.response && error.response.data === "session timedout") {
          console.error("Session timed out. Removing cookie.");
          // Remove session_id cookie
          document.cookie =
            "session_id=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
          setUser(null); // Ensure user is logged out
        } else {
          console.error("Error during authentication:", error.message);
        }
      }
    }
  };
  useEffect(() => {
    checkSession();
  }, []);

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
        setRefreshKey(refreshKey + 1);
      })
      .catch((error) => {
        console.error("Error:", error);
      });
  };
  console.log("selected thread:", selectedThread);

  return (
    <div class="home">
      <div class="sidebar">
        <h2>Retoro</h2>
        <ThreadList onSelectThread={onSelectThread} />
        <UserPanel user={user} handleUser={handleUser} />
      </div>
      {selectedThread ? (
        <div class="content">
          <ThreadView selectedThread={selectedThread} refreshKey={refreshKey} />
          <PostInputDialog onSubmit={onSubmit} />
        </div>
      ) : (
        <div class="content">
          <div class="info">
            <h1>No thread selected.</h1>
          </div>
        </div>
      )}
    </div>
  );
}
