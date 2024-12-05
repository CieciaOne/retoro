import axios from "axios";
import { ChangeEvent, useState } from "react";
import Markdown from "react-markdown";
import { User } from "./UserPanel";

interface UserRegisterDialogProps {
  onRegister: (username: string, password: string) => void;
  hidden: boolean;
}

export function UserRegisterDialog(props: UserRegisterDialogProps) {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [passwordRepeat, setPasswordRepeat] = useState("");

  return (
    <div hidden={props.hidden}>
      <div class="standard-dialog">
        <h2>Register:</h2>
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
        <label class="password">
          <input
            class="standard-input"
            type="password"
            value={passwordRepeat}
            onInput={(event) => {
              setPasswordRepeat(event.currentTarget.value);
            }}
            placeholder={"Repeat your very secure password..."}
          />
        </label>
        <button
          onClick={() => {
            if (password == passwordRepeat)
              props.onRegister(username, password);
            else {
              alert("Passwords don't match");
            }
          }}
          class="standard-button big"
        >
          Register
        </button>
      </div>
    </div>
  );
}
