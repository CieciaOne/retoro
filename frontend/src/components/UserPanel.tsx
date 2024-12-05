import { useState } from "preact/hooks";
import { UserLoginDialog } from "./UserLoginDialog";
import axios from "axios";
import { UserOptionsDialog } from "./UserOptionsDialog";
import { UserRegisterDialog } from "./UserRegisterDialog";

interface UserPanelProps {
  user?: User;
  handleUser: (user: User) => void;
}
export const UserPanel = (props: UserPanelProps) => {
  const [registerHidden, setRegisterHidden] = useState(true);
  const [loginHidden, setLoginHidden] = useState(true);
  const [optionsHidden, setOptionsHidden] = useState(true);
  const [_loading, setLoading] = useState(true);
  const [_error, setError] = useState(null);

  const toggleRegisterDialog = () => {
    setRegisterHidden(!registerHidden);
  };
  const toggleLoginDialog = () => {
    setLoginHidden(!loginHidden);
  };

  const toggleOptionsDialog = () => {
    console.log("");

    setOptionsHidden(!optionsHidden);
  };

  const onLogin = async (username: string, password: string) => {
    try {
      setLoading(true); // Start loading

      const response = await axios.post(
        "http://localhost:8080/api/users/login",
        {
          name: username,
          password: password,
        },
        {
          withCredentials: true, // Ensures the browser stores the session_id cookie
          headers: {
            "Content-Type": "application/json",
          },
        }
      );

      props.handleUser(response.data as User); // Set the logged-in user
      console.debug(response.data); // Log the response data
      setLoginHidden(true);
    } catch (err) {
      setError(err.message); // Save the error
      console.error("Login error:", err); // Log the error for debugging
    } finally {
      setLoading(false); // End loading
    }
  };

  const onRegister = async (username: string, password: string) => {
    try {
      setLoading(true); // Start loading

      const response = await axios.post(
        "http://localhost:8080/api/users/register",
        {
          name: username,
          password: password,
        },
        {
          withCredentials: true, // Ensures the browser stores the session_id cookie
          headers: {
            "Content-Type": "application/json",
          },
        }
      );

      props.handleUser(response.data as User); // Set the logged-in user
      console.debug(response.data); // Log the response data
      setRegisterHidden(true);
    } catch (err) {
      setError(err.message); // Save the error
      console.error("Login error:", err); // Log the error for debugging
    } finally {
      setLoading(false); // End loading
    }
  };

  if (props.user) {
    return (
      <div class="user-panel">
        <hr class="secondary" />
        <h3 class="accent standard-button" onClick={toggleOptionsDialog}>
          {props.user.username}
        </h3>
        <UserOptionsDialog
          handleUser={props.handleUser}
          user={props.user}
          hidden={optionsHidden}
          toggleHidden={toggleOptionsDialog}
        />
      </div>
    );
  } else {
    return (
      <div class="user-panel">
        <hr class="secondary" />
        <h4 class="accent standard-button" onClick={toggleOptionsDialog}>
          Anonymous
        </h4>

        <div style={{ display: "block" }}>
          <h4
            class="inline-block standard-button"
            onClick={toggleRegisterDialog}
          >
            Register
          </h4>

          <h4 class="inline-block"> / </h4>
          <h4 class="inline-block standard-button" onClick={toggleLoginDialog}>
            Login
          </h4>
        </div>
        <UserOptionsDialog
          handleUser={props.handleUser}
          user={props.user}
          hidden={optionsHidden}
          toggleHidden={toggleOptionsDialog}
        />
        <UserLoginDialog onLogin={onLogin} hidden={loginHidden} />
        <UserRegisterDialog onRegister={onRegister} hidden={registerHidden} />
      </div>
    );
  }
};

export interface User {
  id: string;
  username: string;
  created_at: string;
  last_active: string;
}
