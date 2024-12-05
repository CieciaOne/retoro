import axios from "axios";
import { ChangeEvent, useState } from "react";
import Markdown from "react-markdown";
import { User } from "./UserPanel";

interface UserOptionsDialogProps {
  user: User;
  handleUser: (user: User) => void;
  hidden: boolean;
  toggleHidden: () => void;
}

export function UserOptionsDialog(props: UserOptionsDialogProps) {
  function logout() {
    document.cookie =
      "session_id=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";

    props.handleUser(null);
    props.toggleHidden();
  }

  return (
    <div hidden={props.hidden}>
      <div class="standard-dialog">
        <h2>Options:</h2>

        {props.user && (
          <button onClick={() => logout()} class="standard-button big">
            Log out
          </button>
        )}
      </div>
    </div>
  );
}
