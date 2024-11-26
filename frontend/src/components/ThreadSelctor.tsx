export const ThreadSelector = ({ thread, onSelectThread }) => {
    const formatedDate = new Date(thread.created_at).toLocaleDateString();
    return(
    <div className="thread" onClick={() => onSelectThread(thread.id)}>
      <h4 onClick={onSelectThread(thread.id)}>{thread.name}</h4>
    </div>
  );
}
  
interface Thread{
    id:  string,
    name: string,
    createdAt: Date,

}
