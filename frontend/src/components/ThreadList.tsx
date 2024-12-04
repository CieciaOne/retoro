import axios from "axios";
import { useEffect, useState } from "preact/hooks";
import { Thread, ThreadSelector } from "./ThreadSelctor";

interface ThreadListProps {
  onSelectThread: (thread: Thread) => void;
  selectedThread: string | null;
}
export const ThreadList = (props: ThreadListProps) => {
  const [threads, setThreads] = useState(null);
  const [threadsRefreshKey, setThreadsRefreshKey] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const fetchThreads = async () => {
    try {
      setLoading(true); // Start loading
      const response = await axios.get("http://localhost:8080/api/threads");
      if (response.data != threads) {
        setThreads(response.data); // Save the data
      }
    } catch (err) {
      setError(err.message); // Save the error
    } finally {
      setLoading(false); // End loading
    }
  };

  useEffect(() => {
    fetchThreads();
    const interval = setInterval(() => {
      setThreadsRefreshKey((prev) => prev + 1);
    }, 60000); // 1 min
    return () => clearInterval(interval); // Clean up interval
  }, []);

  useEffect(() => {
    fetchThreads();
  }, [threadsRefreshKey]);

  // Conditional rendering based on state
  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error}</p>;
  return (
    <div class="thread-selector">
      <hr class="secondary" />
      {threads.map((thread: Thread) => {
        return (
          <ThreadSelector
            selected={thread.id == props.selectedThread}
            thread={thread}
            onSelectThread={props.onSelectThread}
          />
        );
      })}
      {/* <NewThread */}
    </div>
  );
};
