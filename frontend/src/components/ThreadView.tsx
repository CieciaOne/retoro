import axios from "axios";
import { useEffect, useState } from "preact/hooks";
import { Post, PostView } from "./PostView";
import { Thread } from "./ThreadSelctor";

interface ThreadProps {
  selectedThread: Thread;
  refreshKey: number;
}

export function ThreadView(props: ThreadProps) {
  const [posts, setPosts] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const fetchPosts = async () => {
    try {
      setLoading(true); // Start loading
      const response = await axios.get(
        `http://localhost:8080/api/posts?thread=${props.selectedThread.id}`
      );
      if (response.data != posts) {
        setPosts(response.data); // Save the data
      }
    } catch (err) {
      setError(err.message); // Save the error
    } finally {
      setLoading(false); // End loading
    }
  };
  useEffect(() => {
    fetchPosts();
  }, [props.selectedThread, props.refreshKey]); // Empty dependency array means this runs once on mount

  // Conditional rendering based on state
  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error}</p>;
  if (posts == undefined || posts.length == 0)
    return (
      <div>
        {/* <h2>{props.selectedThread.name}</h2> */}
        <hr class="secondary" />
        <p>No posts in thread</p>
      </div>
    );
  return (
    <div>
      {/* <h2>{props.selectedThread.name}</h2> */}
      <hr class="secondary" />

      {posts &&
        posts.map((post) => {
          return <PostView post={post as Post} />;
        })}
    </div>
  );
}
