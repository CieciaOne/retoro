import { ChangeEvent, useState } from "react";
import Markdown from "react-markdown";

interface PostInputDialogProps {
  onSubmit: (content: string) => void;
}

export function PostInputDialog({ onSubmit }: PostInputDialogProps) {
  const [content, setContent] = useState("");
  const [checked, setChecked] = useState(false);

  const handleSubmit = async () => {
    await onSubmit(content);
  };

  return (
    <div class="post-input">
      {checked ? (
        <div class="post-input-area">
          <Markdown>{content}</Markdown>
        </div>
      ) : (
        <textarea
          class="post-input-area"
          value={content}
          placeholder={"What are you thinking about...?"}
          onInput={(e) => setContent(e.currentTarget.value)}
        />
      )}
      <div>
        <label class="switch">
          <input
            class="standard-checkbox"
            type="checkbox"
            checked={checked}
            onInput={() => setChecked(!checked)}
          />
          Preview
        </label>
        <button onClick={handleSubmit} class="big standard-button float-right">
          Post
        </button>
      </div>
    </div>
  );
}
