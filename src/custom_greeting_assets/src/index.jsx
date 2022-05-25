import * as React from "react";
import { render } from "react-dom";
import { service } from "../../declarations/service";
import { Principal } from "@dfinity/principal";

const Create = () => {
  const [canisterId, setCanisterId] = React.useState('');
  const [email, setEmail] = React.useState('');
  const [threshold, setThreshold] = React.useState('');
  const [message, setMessage] = React.useState('');

  async function createUser() {
    const greeting = await service.create({ "threshold": threshold, canister_id: canisterId, "email": email });
    setMessage("succeeded");
  }

  return (
    <div style={{ "fontSize": "30px" }}>
      <div style={{ "backgroundColor": "yellow" }}>
        <p>Dfinity DDos Defender</p>
        <p>
          Please Input Your threshold, canister_id, email, and click
          <b> Submit</b> to display the result.
        </p>
      </div>
      <div style={{ margin: "30px" }}>
        <input
          id="threshold"
          value={threshold}
          onChange={(ev) => setThreshold(ev.target.value)}
        ></input>
        <input
          id="canister_id"
          value={canisterId}
          onChange={(ev) => setCanisterId(Principal.fromText(ev.target.value))}
        ></input>
        <input
          id="email"
          value={email}
          onChange={(ev) => setEmail(ev.target.value)}
        ></input>
        <button onClick={createUser}>Submit!</button>
      </div>
      <div>
        Result is: "
        <span style={{ color: "blue" }}>{message}</span>"
      </div>
    </div>
  );
};

render(<Create />, document.getElementById("app"));
