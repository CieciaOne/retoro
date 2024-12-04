import axios from "axios";
import { useEffect, useState } from "preact/hooks";
import { Post } from "./Post";
import { Thread } from "./ThreadSelctor";

interface ThreadProps {
  selectedThread: Thread;
  refreshKey: number;
}

export function ThreadView(props: ThreadProps) {
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    // Define an async function to fetch data
    const fetchData = async () => {
      try {
        setLoading(true); // Start loading
        const response = await axios.get(
          `http://localhost:8080/api/posts?thread=${props.selectedThread.id}`
        );
        setData(response.data); // Save the data

        console.log("Posts data:", data);
      } catch (err) {
        setError(err.message); // Save the error
      } finally {
        setLoading(false); // End loading
      }
    };

    fetchData();
  }, [props.selectedThread, props.refreshKey]); // Empty dependency array means this runs once on mount

  // Conditional rendering based on state
  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error}</p>;
  if (data == undefined || data.length == 0)
    return (
      <div>
        <h2>{props.selectedThread.name}</h2>
        <hr class="secondary" />
        <p>No posts in thread</p>
      </div>
    );
  return (
    <div>
      <h2>{props.selectedThread.name}</h2>
      <hr class="secondary" />

      {data &&
        data.map((post) => {
          return <Post post={post} />;
        })}
    </div>
  );
}
