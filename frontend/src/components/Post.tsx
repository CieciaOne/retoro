export const Post = ({  post }) => (
    <div className="post" onClick={() => onSelectThread(post.id)}>
      <h4>{post.author}</h4>
      <div>{post.description}</div>
      <div>{post.description}</div>
      <div>{post.description}</div>
    </div>
  );

function onSelectThread(id: any): void {

    throw new Error("Function not implemented.");
}
