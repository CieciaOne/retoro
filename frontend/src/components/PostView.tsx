import Markdown from "react-markdown";

interface PostProps {
  post: Post;
}
export function PostView(props: PostProps) {
  const formatedDate = new Date(props.post.created_at).toLocaleString();
  return (
    <div id={props.post.id} class="post">
      <div class="post-header">
        <h4 class="author">{`${
          props.post.author_id == null ? "Anonymous" : props.post.author_name
        }`}</h4>
        <h6 class="datetime">{formatedDate}</h6>
      </div>
      <hr />
      <div>
        <Markdown>{props.post.content}</Markdown>
      </div>
    </div>
  );
}

export interface Post {
  author_id: string;
  author_name: string;
  content: string;
  created_at: Date;
  id: string;
  thread_id: string;
}
