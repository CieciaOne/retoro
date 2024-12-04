import axios from "axios";
import { ChangeEvent, useState } from "react";
import Markdown from "react-markdown";
import { User } from "./UserPanel";

interface UserOptionsDialogProps {
  user: User;
  handleUser: (user: User) => void;
  hidden: boolean;
}

export function UserOptionsDialog(props: UserOptionsDialogProps) {
  function toggleTheme() {
    var element = document.body;
    element.classList.toggle("dark-mode");
  }

  function logout() {
    document.cookie =
      "session_id=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";

    props.handleUser(null);
  }

  return (
    <div hidden={props.hidden}>
      <div class="login-dialog">
        <h2>Options:</h2>
        <button onClick={() => toggleTheme()} class="standard-button big">
          Toggle theme
        </button>

        <button onClick={() => logout()} class="standard-button big">
          Log out
        </button>
      </div>
    </div>
  );
}
