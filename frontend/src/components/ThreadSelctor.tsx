interface ThreadSelectorProps {
  thread: Thread;
  selected: boolean;
  onSelectThread: (thread: Thread) => void;
}
export const ThreadSelector = (props: ThreadSelectorProps) => {
  return props.selected ? (
    <div
      id={props.thread.id}
      class="thread selected"
      onClick={() => props.onSelectThread(props.thread)}
    >
      <h4>{props.thread.name}</h4>
    </div>
  ) : (
    <div
      id={props.thread.id}
      class="thread"
      onClick={() => props.onSelectThread(props.thread)}
    >
      <h4>{props.thread.name}</h4>
    </div>
  );
};

export interface Thread {
  id: string;
  name: string;
  created_at: Date;
  last_active: Date;
}
