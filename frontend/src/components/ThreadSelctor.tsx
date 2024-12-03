interface ThreadSelectorProps {
  thread: Thread;
  onSelectThread: (thread: Thread) => void;
}
export const ThreadSelector = (props: ThreadSelectorProps) => {
  // const formatedDate = new Date(thread.created_at).toLocaleDateString();
  return (
    <div class="thread" onClick={() => props.onSelectThread(props.thread)}>
      <h4>{props.thread.name}</h4>
    </div>
  );
};

export interface Thread {
  id: string;
  name: string;
  createdAt: Date;
}
