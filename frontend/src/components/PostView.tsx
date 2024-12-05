import Markdown from "react-markdown";

interface PostProps {
  post: Post;
}
export function PostView(props: PostProps) {
  // const date = new Date(props.post.created_at).toLocaleTimeString();
  const formatedDate = new Date(props.post.created_at).toLocaleString();
  return (
    <div id={props.post.id} class="post">
      <div class="post-header">
        <h4 class="author">{`${
          props.post.author_id == null ? "Anonymous" : props.post.author_name
        }`}</h4>
        {/* <h6 class="tag">{`#${post.id.slice(0, 8).toUpperCase()}`}</h6> */}
        <h6 class="tag">{formatedDate}</h6>
      </div>
      <div>
        <Markdown>{props.post.content}</Markdown>
      </div>
    </div>
  );
}

function onSelectThread(id: any): void {
  throw new Error("Function not implemented.");
}

export interface Post {
  author_id: string;
  author_name: string;
  content: string;
  created_at: Date;
  id: "8434b860-c705-4bfa-9c8c-445a9e65f298";
  thread_id: "c7d0db50-f925-4c4f-8247-c82f3da11b88";
}
