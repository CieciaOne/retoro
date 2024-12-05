import axios from "axios";
import { ChangeEvent, useState } from "react";
import Markdown from "react-markdown";
import { User } from "./UserPanel";

interface UserLoginDialogProps {
  onLogin: (username: string, password: string) => void;
  hidden: boolean;
}

export function UserLoginDialog(props: UserLoginDialogProps) {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  return (
    <div hidden={props.hidden}>
      <div class="standard-dialog">
        <h2>Log-in:</h2>
        <label class="login">
          <input
            class="standard-input"
            type="text"
            value={username}
            onInput={(event) => {
              setUsername(event.currentTarget.value);
            }}
            placeholder={"Your username..."}
          />
        </label>
        <br />
        <label class="password">
          <input
            class="standard-input"
            type="password"
            value={password}
            onInput={(event) => {
              setPassword(event.currentTarget.value);
            }}
            placeholder={"Your very secure password..."}
          />
        </label>
        <br />
        <button
          onClick={() => {
            props.onLogin(username, password);
          }}
          class="standard-button big"
        >
          Login
        </button>
      </div>
    </div>
  );
}
