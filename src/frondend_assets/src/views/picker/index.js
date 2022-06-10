import React from 'react';
import { Principal } from "@dfinity/principal";
import { service } from "../../../../declarations/service";

const Picker = (actor, principalId) => {
  const [canisterId, setCanisterId] = React.useState('');
  const [email, setEmail] = React.useState('');
  const [threshold, setThreshold] = React.useState('');
  const [message, setMessage] = React.useState('');

  async function createUser() {
    try {
      console.log(canisterId);
      const principal = await window.ic.plug.agent.getPrincipal();
      const controller = await service.getController({ 'canister_id': canisterId });
      let temp = 0;
      for (let i = 0; i < controller.length; i++) {
        if (JSON.stringify(controller[i]) !== JSON.stringify(principal)) {
          temp++;
        }
      }
      if (temp === controller.length) {
        setMessage(" : Failed, because you are not a controller of this canister");
        throw 'You are not a controller';
      }
      const params = {
        to: 'ezw55-al2r4-u5pm6-jaew5-43qve-46acg-ypjdh-caeh4-3iv3o-eh5qw-kae',
        amount: 2_000_000,
        memo: 'test for dddd',
      };
      const result = await window.ic.plug.requestTransfer(params);
      console.log(result);
      const greeting = await service.create({ "threshold": threshold, canister_id: canisterId, "email": email });
      setMessage(" : succeed");
    } catch (e) {
      console.log(e)
      setMessage(" : Failed, you have to send ICP to subscribe our service");
    }
  }

  return (
    <div className='leaderboard-container' style={{ "fontSize": "30px" }}>
      <div align="center">
        <font face="verdana" size="6" color="black">Please submit your information to subscribe our service.</font>
      </div>
      <div style={{ margin: "30px" }} align="center">
        <font face="verdana" size="4" color="black">threshold:  </font>
        <input
          id="threshold"
          value={threshold}
          onChange={(ev) => setThreshold(ev.target.value)}
        ></input>
        <font face="verdana" size="4" color="black">     canister id:  </font>
        <input
          id="canister_id"
          value={canisterId}
          onChange={(ev) => setCanisterId(Principal.fromText(ev.target.value))}
        ></input>
        <font face="verdana" size="4" color="black">     email:  </font>
        <input
          id="email"
          value={email}
          onChange={(ev) => setEmail(ev.target.value)}
        ></input>
        <button onClick={createUser}>Submit!</button>
      </div>
      <div align="center">
        <font face="verdana" size="5" color="black">
          <span style={{}}>Result {message}</span>
        </font>
      </div>
    </div >
  );
};

export default Picker;
