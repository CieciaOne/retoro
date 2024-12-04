import Markdown from "react-markdown";

export const Post = ({ post }) => (
  <div id={post.id} class="post" onClick={() => onSelectThread(post.id)}>
    <div class="post-header">
      <h4 class="author">{`${
        post.author_id == null ? "Anonymous" : post.author_name
      }`}</h4>
      <h6 class="tag">{`#${post.id.slice(0, 8).toUpperCase()}`}</h6>
    </div>
    <div>
      <Markdown>{post.content}</Markdown>
    </div>
    <div>{post.description}</div>
  </div>
);

function onSelectThread(id: any): void {
  throw new Error("Function not implemented.");
}
