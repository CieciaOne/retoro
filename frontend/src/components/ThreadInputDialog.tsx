import axios from "axios";
import { ChangeEvent, useState } from "react";
import Markdown from "react-markdown";
import { Thread } from "./ThreadSelctor";

interface ThreadInputDialogProps {
  toggleHidden: () => void;
  hidden: boolean;
  refreshThreads: () => void;
  selectThread: (thread: Thread) => void;
}

export function ThreadInputDialog(props: ThreadInputDialogProps) {
  const [newThread, setNewThread] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const handleNewThread = (name: string) => {
    setNewThread(name);
  };
  const addThread = async () => {
    try {
      setLoading(true); // Start loading
      if (newThread != "" && newThread != null) {
        const response = await axios.post("http://localhost:8080/api/threads", {
          name: newThread,
        });
        console.log(response.data as Thread);

        console.log(response);
        props.refreshThreads();
        props.toggleHidden();
        props.selectThread(response.data as Thread);
      }
    } catch (err) {
      setError(err.message); // Save the error
    } finally {
      setLoading(false); // End loading
    }
  };

  return (
    <div hidden={props.hidden}>
      <div class="standard-dialog">
        <h2>Add new thread:</h2>
        <input
          class="standard-input"
          type="text"
          value={newThread}
          onInput={(v) => handleNewThread(v.currentTarget.value)}
        />
        <div>
          <button class="standard-button big" onClick={addThread}>
            Add thread
          </button>
        </div>
      </div>
    </div>
  );
}
